use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};
use std::time::{Duration, Instant};
use std::error;

use serde::{Deserialize, Serialize};

use uuid;

use futures::{SinkExt, StreamExt};
use futures::stream::SplitSink;

use tokio::sync::{mpsc, Mutex};
use tokio::net::TcpStream;
use tokio;

use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, Connector, connect_async_tls_with_config, tungstenite, tungstenite::protocol::Message};

use rustls::ClientConfig;
use rustls::client::danger::{ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{CryptoProvider, verify_tls12_signature, verify_tls13_signature};

use crate::network::protocol::{ServerMessage, ClientMessage};
use crate::components::main_window;

#[derive(Debug)]
pub struct Client {
    pub id: uuid::Uuid,
    pub id_on_server: Mutex<Option<uuid::Uuid>>,
    pub url: Mutex<Option<String>>,
    pub creation: Instant,
    pub connected: Mutex<bool>,
    pub users: Mutex<HashMap<uuid::Uuid, User>>,
    pub display: Mutex<Vec<DisplayMessage>>,
    pub nickname: Mutex<String>,

    pub tx: Mutex<Option<mpsc::UnboundedSender<ClientMessage>>>,
    sink: Mutex<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    config: ClientConfig,
}

#[derive(Clone)]
pub struct ClientData {
    pub id: uuid::Uuid,
    pub id_on_server: Option<uuid::Uuid>,
    pub creation: Instant,
    pub connected: bool,
    pub url: Option<String>,
    pub users: HashMap<uuid::Uuid, User>,
    pub display: Vec<DisplayMessage>,
    pub nickname: String,
    pub tx: Option<mpsc::UnboundedSender<ClientMessage>>,
}

impl PartialEq for ClientData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.id_on_server == other.id_on_server &&
        self.creation == other.creation &&
        self.connected == other.connected &&
        self.url == other.url &&
        self.users == other.users &&
        self.display == other.display &&
        self.nickname == other.nickname
    }
}

impl Debug for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.nickname, self.display)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub struct User {
    pub id: uuid::Uuid,
    pub nickname: String,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum DisplayMessage {
    Text(uuid::Uuid, String, String),
    NowKnownAs(uuid::Uuid, String, String),
    NicknameTaken(String),
    Joined(uuid::Uuid, String),
    Left(uuid::Uuid, String),
    Pong(uuid::Uuid, Duration),
}

#[derive(Debug)]
struct AcceptAnyCert(Arc<CryptoProvider>);

impl ServerCertVerifier for AcceptAnyCert {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error>
    {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error>
    {
        verify_tls12_signature(message, cert, dss, &self.0.signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &rustls::pki_types::CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error>
    {
        verify_tls13_signature(message, cert, dss, &self.0.signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
       self.0.signature_verification_algorithms.supported_schemes()
    }
}


#[allow(dead_code)]
fn err<S1, S2>(tx: &main_window::Sender, title: S1, message: S2) where S1: Into<String>, S2: Into<String> {
    let title: String = title.into();
    let message: String = message.into();
    let window_msg = main_window::Message::err(title, message);
    let _ = tx.send(window_msg);
}

#[allow(dead_code)]
fn log<S1, S2>(tx: &main_window::Sender, title: S1, message: S2) where S1: Into<String>, S2: Into<String> {
    let title: String = title.into();
    let message: String = message.into();
    let window_msg = main_window::Message::log(title, message);
    let _ = tx.send(window_msg);
}

fn make_config() -> Result<ClientConfig, Box<dyn error::Error>> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());

    Ok(
        ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()?
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AcceptAnyCert(provider)))
            .with_no_client_auth()
    )
}

impl Drop for Client {
    fn drop(&mut self) {
       println!("Client dropped");
    }
}

impl Client {
    pub fn new() -> Result<Arc<Self>, Box<dyn error::Error>> {
        Ok(Arc::new(Client {
            id: uuid::Uuid::new_v4(),
            id_on_server: None.into(),
            url: None.into(),
            creation: Instant::now(),
            connected: false.into(),
            users: HashMap::new().into(),
            display: vec![].into(),
            nickname: "".to_string().into(),
            tx: None.into(),
            sink: None.into(),
            config: make_config()?,
        }))
    }

    pub async fn get_data(&self) -> ClientData {
        ClientData {
            id: self.id,
            id_on_server: self.id_on_server.lock().await.clone(),
            url: self.url.lock().await.clone(),
            creation: self.creation,
            connected: *self.connected.lock().await,
            users: self.users.lock().await.clone(),
            display: self.display.lock().await.clone(),
            nickname: self.nickname.lock().await.clone(),
            tx: self.tx.lock().await.clone(),
        }
    }

    pub async fn run(&self, window_tx: main_window::Sender, url: &String, nickname: &String) -> Result<main_window::Message, tungstenite::Error> {
        let (tx, mut rx) = mpsc::unbounded_channel::<ClientMessage>();

        let url = if !url.starts_with("wss://") { "wss://".to_string() + url } else { url.to_owned() };

        *self.tx.lock().await = Some(tx);

        *self.url.lock().await = Some(url.to_owned());

        let (ws_stream, _) = connect_async_tls_with_config(
            &url,
            None,
            false,
            Some(Connector::Rustls(Arc::new(self.config.clone()))),
        ).await?;

        *self.connected.lock().await = true;

        let (sink, mut source) = ws_stream.split();

        *self.sink.lock().await = Some(sink);

        *self.nickname.lock().await = nickname.clone();

        self.send_join(nickname).await
            .ok_or(tungstenite::Error::ConnectionClosed)?;

        self.request_flush(&window_tx);

        let process_sink = async |client_msg| {
            if self.send(client_msg).await.is_none() {
                self.disconnect().await;
                self.request_flush(&window_tx);
                Err(tungstenite::Error::ConnectionClosed)?;
            }

            Ok::<(), tungstenite::Error>(())
        };

        let process_source = async |raw_msg| {
            match raw_msg {
                Some(Ok(Message::Binary(data))) => {
                    let res = match data.try_into().map_err(|_| tungstenite::Error::ConnectionClosed)? {
                        ServerMessage::Welcome(new_id) => {
                            self.on_welcome(&new_id).await;

                            Ok(None)
                        },
                        ServerMessage::NicknameChanged(id, new_nickname) => {
                            self.on_nickname_change(id, &new_nickname).await
                        },
                        ServerMessage::NicknameAlreadyTaken(nickname) => {
                            Ok(Some(DisplayMessage::NicknameTaken(nickname.clone())))
                        },
                        ServerMessage::TextMessage(id, text) => {
                            self.on_text_message(id, &text).await
                        },
                        ServerMessage::UserLeft(id) => {
                            self.on_user_left(id).await
                        },
                        ServerMessage::UserJoined(user) => {
                            Ok(self.on_user_joined(user).await)
                        },
                        ServerMessage::CurrentUsers(users) => {
                            Ok(self.on_current_users(users).await)
                        },
                        ServerMessage::Pong => Ok(None),
                        ServerMessage::NoNicknameError => Err(
                            main_window::Error::err(
                                "Server error",
                                "Nickname not set"
                            )
                        ),
                    };

                    match res {
                        Ok(Some(display_msg)) => {
                            (*self.display.lock().await).push(display_msg);

                            self.request_flush(&window_tx);
                        },
                        Err(window_err) => {
                            let _ = window_tx.send(window_err.into());
                        },
                        Ok(None) => {},
                    }
                },
                Some(Ok(_)) => {}, // non-binary frames are ignored
                Some(Err(connection_err)) => Err(connection_err)?, // connection error
                None => Err(tungstenite::Error::AlreadyClosed)?, // server disconnected
            };

            Ok::<(), tungstenite::Error>(())
        };

        loop {
            tokio::select! {
                Some(client_msg) = rx.recv() => process_sink(client_msg).await?,
                raw_msg = source.next() => process_source(raw_msg).await?,
            }
        }
    }

    pub fn request_flush(&self, window_tx: &main_window::Sender) {
        let _ = window_tx.send(main_window::Message::ClientShouldFlush(self.id));
    }

    pub async fn on_welcome(&self, new_id: &uuid::Uuid) {
        *self.id_on_server.lock().await = Some(new_id.clone());

    }

    pub async fn on_current_users(&self, users: Vec<User>) -> Option<DisplayMessage> {
        let users = users.iter().map(|u| (u.id, u.to_owned()));

        *self.users.lock().await = users.collect();

        None
    }

    pub async fn on_nickname_change(&self, id: uuid::Uuid, new_nickname: &String) -> Result<Option<DisplayMessage>, main_window::Error> {
        let old_nickname: String;

        {
            let mut guard = self.users.lock().await;
            let user = guard.get_mut(&id)
                .ok_or(
                    main_window::Error::log(
                        "Unsynced userlist",
                        "Received a nickname change from an unknown user"
                    )
                )?;

            old_nickname = user.nickname.clone();
            user.nickname = new_nickname.clone();
        }

        if let Some(server_id) = *self.id_on_server.lock().await && server_id == id {
            *self.nickname.lock().await = new_nickname.clone();
        }

        Ok(Some(DisplayMessage::NowKnownAs(id, old_nickname, new_nickname.clone())))
    }

    pub async fn on_text_message(&self, id: uuid::Uuid, text: &String) -> Result<Option<DisplayMessage>, main_window::Error> {
        self.users.lock().await.get(&id)
            .map(|user| Some(DisplayMessage::Text(id.to_owned(), user.nickname.clone(), text.clone())))
            .ok_or(
                main_window::Error::log(
                    "Unsynced userlist",
                    "Received a message from an unknown user"
                )
            )
    }

    pub async fn on_user_joined(&self, user: User) -> Option<DisplayMessage> {
        let nickname = user.nickname.clone();
        let id = user.id;
        self.users.lock().await.insert(user.id, user);

        Some(DisplayMessage::Joined(id, nickname))
    }

    pub async fn on_user_left(&self, id: uuid::Uuid) -> Result<Option<DisplayMessage>, main_window::Error> {
        self.users.lock().await.remove(&id)
            .map(|user| Some(DisplayMessage::Left(id, user.nickname)))
            .ok_or(
                main_window::Error::log(
                    "Unsynced userlist",
                    "Received a left event from an unknown user"
                )
            )
    }

    pub async fn send_join(&self, nickname: &String) -> Option<()> {
        self.send(ClientMessage::Join(nickname.clone())).await
    }

    pub async fn send_text(&self, text: &String) -> Option<()> {
        self.send(ClientMessage::SendMessage(text.clone())).await
    }

    pub async fn send_set_nickname(&self, new_nickname: &String) -> Option<()> {
        self.send(ClientMessage::SetNickname(new_nickname.clone())).await
    }

    pub async fn send_quit(&self) -> Option<()> {
        self.send(ClientMessage::Quit).await;
        None
    }

    pub async fn disconnect(&self) -> Option<()> {
        *self.connected.lock().await = false;
        *self.url.lock().await = None;
        *self.tx.lock().await = None;
        *self.sink.lock().await = None;
        *self.id_on_server.lock().await = None;
        Some(())
    }


    pub async fn send(&self, message: ClientMessage) -> Option<()> {
        let mut sink = self.sink.lock().await;

        if let Some(ref mut sink) = *sink {
            sink.send(Message::Binary(message.clone().into())).await.ok()?;

            match message {
                ClientMessage::Quit => None,
                _ => Some(())
            }
        }
        else {
            None
        }
    }
}
