use std::{collections::HashMap, collections::HashSet, sync::Arc};
use std::net::SocketAddr;
use std::error;

use futures::{SinkExt, StreamExt, TryStreamExt};
use rcgen::{generate_simple_self_signed, CertifiedKey};
use rustls::ServerConfig;
use rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio::{net::TcpListener, sync::Mutex, net::TcpStream, self};
use tokio::sync::mpsc::{error::SendError, self};
use tokio_rustls::TlsAcceptor;
use uuid::self;

use tokio_tungstenite::{tungstenite::protocol::Message, tungstenite, accept_async};

use crate::network::protocol::ServerMessage;
use crate::network::{protocol, client};
use crate::components::main_window;
use crate::tokio_handle;

pub struct Server {
    pub id: uuid::Uuid,
    pub listener: TcpListener,

    acceptor: TlsAcceptor,
    clients: Mutex<HashMap<uuid::Uuid, Client>>,

}

impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
       self.id == other.id
    }
}

#[derive(Clone)]
pub struct Client {
    pub id: uuid::Uuid,
    pub address: SocketAddr,
    pub nickname: Option<String>,

    tx: mpsc::UnboundedSender<protocol::ServerMessage>,
}

impl TryInto<client::User> for &Client {
    type Error = ();

    fn try_into(self) -> Result<client::User, Self::Error> {
        self.nickname.clone()
            .map_or(Err(()), |nickname| Ok(
                client::User {
                    id: self.id,
                    nickname: nickname,
                }
            ))
    }
}

fn err<S1, S2>(tx: &main_window::Sender, title: S1, message: S2) where S1: Into<String>, S2: Into<String> {
    let title: String = title.into();
    let message: String = message.into();
    let window_msg = main_window::Message::err(title, message);
    let _ = tx.send(window_msg);
}

fn log<S1, S2>(tx: &main_window::Sender, title: S1, message: S2) where S1: Into<String>, S2: Into<String> {
    let title: String = title.into();
    let message: String = message.into();
    let window_msg = main_window::Message::log(title, message);
    let _ = tx.send(window_msg);
}

fn make_acceptor() -> Result<TlsAcceptor, Box<dyn error::Error>> {
    let CertifiedKey { cert, signing_key } = generate_simple_self_signed(vec!["localhost".to_string()])?;
    let cert_der = cert.der().clone();
    let key_der = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(signing_key));
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let config = ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], key_der)?;

    Ok(TlsAcceptor::from(Arc::new(config)))
}

impl Server {
    pub async fn new(url: String) -> Result<Arc<Self>, Box<dyn error::Error>> {
        let listener = TcpListener::bind(&url).await?;

        Ok(Arc::new(Server {
            id: uuid::Uuid::new_v4(),
            clients: Mutex::new(HashMap::new()),
            acceptor: make_acceptor()?,
            listener: listener,
        }))
    }

    pub async fn listen(self: Arc<Self>, window_tx: main_window::Sender) -> main_window::Message {
        loop {
            match self.listener.accept().await {
                Ok((stream, address)) => {
                    tokio_handle().spawn(
                        self.clone().on_client_connected(
                            stream,
                            address,
                            window_tx.clone(),
                        )
                    );
                },
                Err(e) => {
                    err(&window_tx, "Connection error", format!("{e:?}"));
                },
            }
        }
    }

    pub async fn on_client_connected(self: Arc<Self>, stream: TcpStream, addr: SocketAddr, window_tx: main_window::Sender) {
        let tls = match self.acceptor.accept(stream).await {
            Ok(tls) => tls,
            Err(e) => {
                err(&window_tx, "TLS handshake failed", format!("{e:?}"));
                return;
            },
        };
        let ws = match accept_async(tls).await {
            Ok(ws) => ws,
            Err(e) => {
                err(&window_tx, "WS handshake failed", format!("{e:?}"));
                return;
            },
        };

        let (tx, mut rx) = mpsc::unbounded_channel::<protocol::ServerMessage>();

        let id = uuid::Uuid::new_v4();

        let client = Client {
            id: id,
            address: addr,
            nickname: None,
            tx: tx,
        };

        self.clients.lock().await.insert(id, client);

        let (mut sink, source) = ws.split();

        let _ = self.send_to(&id, ServerMessage::Welcome(id)).await;

        let process_source = source.try_for_each(async |msg| {
            if let Message::Binary(msg) = &msg {
                match msg.try_into() {
                    Ok(client_msg) => {
                        let result = match &client_msg {
                            protocol::ClientMessage::Join(nickname) => {
                                self.on_user_joined(&id, nickname).await
                            },
                            protocol::ClientMessage::SetNickname(nickname) => {
                                self.on_user_set_nickname(&id, nickname, &client_msg).await;

                                Some(())
                            },
                            protocol::ClientMessage::SendMessage(_) => {
                                self.on_user_send_message(&id, &client_msg).await;

                                Some(())
                            },
                            protocol::ClientMessage::Ping => {
                                let server_msg = client_msg.forward(&id)
                                    .expect("Ping should always forward");

                                self.send_to(&id, server_msg).await.ok()
                            },
                            protocol::ClientMessage::Quit => {
                                self.on_user_quit(&id, &client_msg).await;
                                None
                            },
                        };

                        result.ok_or(tungstenite::Error::AlreadyClosed)?;
                    },
                    Err(err) => {
                        log(&window_tx, "Deserialization error", format!("{addr} sent a non msgpack message {err}"));

                        return Err(tungstenite::Error::AlreadyClosed);
                    }
                }
            }

            Ok(())
        });

        let process_sink = async {
            while let Some(msg) = rx.recv().await {
                let _ = sink.send(Message::Binary(msg.into())).await;
            }
        };

        tokio::select! {
            _ = process_sink => {},
            _ = process_source => {},
        }

        let _ = window_tx.send(main_window::Message::ServerClientDisconnected(addr));

        log(&window_tx, "Disconnected", format!("{id} ({addr}) disconnected"));

        self.clients.lock().await.remove(&id);
    }

    async fn try_change_nickname(&self, id: &uuid::Uuid, nickname: &String) -> Option<()> {
        let changed = {
            let mut clients = self.clients.lock().await;

            let taken = clients.iter()
                .filter_map(|(_, v)| v.nickname.clone())
                .collect::<HashSet<String>>()
                .contains(nickname);

            if !taken && let Some(ref mut myself) = clients.get_mut(&id) {
                myself.nickname = Some(nickname.clone());
            }

            !taken
        };

        if changed { Some(()) }
        else {
            let _ = self.send_to(&id, ServerMessage::NicknameAlreadyTaken(nickname.clone())).await;
            None
        }
    }

    pub async fn broadcast(&self, message: protocol::ServerMessage) {
        for (_, other) in self.clients.lock().await.iter() {
            let _ = other.tx.send(message.clone());
        }
    }

    pub async fn broadcast_except(&self, except: &uuid::Uuid, message: protocol::ServerMessage) {
        for (_, other) in self.clients.lock().await.iter().filter(|(k, _)| k != &except) {
            let _ = other.tx.send(message.clone());
        }
    }

    pub async fn send_to(&self, to: &uuid::Uuid, message: protocol::ServerMessage) -> Result<(), SendError<ServerMessage>> {
        if let Some(client) = self.get_client(to).await {
            client.tx.send(message.clone())?;

            return Ok(())
        }

        Err(mpsc::error::SendError(message))
    }

    pub async fn get_client(&self, id: &uuid::Uuid) -> Option<Client> {
        self.clients.lock().await.get(id).map(|c| c.to_owned())
    }

    pub async fn get_users(&self) -> Vec<client::User> {
        self.clients.lock().await.iter()
            .filter_map(|(_, v)| v.try_into().ok())
            .collect()
    }

    pub async fn get_user(&self, id: &uuid::Uuid) -> Option<client::User> {
        self.get_client(id).await.and_then(|c| (&c).try_into().ok())
    }

    pub async fn on_user_joined(&self, id: &uuid::Uuid, nickname: &String) -> Option<()> {
        self.try_change_nickname(id, nickname).await?;

        let updated_user = self.get_user(&id).await?;

        let joined_msg = protocol::ServerMessage::UserJoined(updated_user);
        let userlist_msg = protocol::ServerMessage::CurrentUsers(self.get_users().await);

        self.broadcast(joined_msg).await;

        self.send_to(&id, userlist_msg).await.ok()
    }

    pub async fn on_user_set_nickname(&self, id: &uuid::Uuid, nickname: &String, client_msg: &protocol::ClientMessage) {
        if self.try_change_nickname(id, nickname).await.is_none() {
            return;
        }

        let server_msg = client_msg.forward(&id)
            .expect("SetNickname should always forward");

        self.broadcast(server_msg).await;
    }

    pub async fn on_user_send_message(&self, id: &uuid::Uuid, client_msg: &protocol::ClientMessage) -> Option<()> {
        if self.get_client(&id).await.is_some() {
            let server_msg = client_msg.forward(&id)
                .expect("SendMessage should always forward");

            // Send back to sender so they know their message arrived at the server.
            self.broadcast(server_msg).await;
        }
        else {
            self.send_to(&id , protocol::ServerMessage::NoNicknameError).await.ok()?;
        }

        Some(())
    }

    pub async fn on_user_quit(&self, id: &uuid::Uuid, client_msg: &protocol::ClientMessage) {
        let server_msg = client_msg.forward(&id)
            .expect("Quit should always forward");

        self.broadcast_except(&id , server_msg).await;
    }
}

