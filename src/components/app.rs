use std::collections::{HashMap, VecDeque};
use std::net::SocketAddr;
use std::sync::{Arc, OnceLock};

use tokio::sync::{Mutex, mpsc};
use tokio_util::sync::CancellationToken;
use uuid;

use windows_reactor::*;

use crate::components::main_window::{self, MainWindow};
use crate::extensions::*;
use crate::network::{client, server};
use crate::helpers::styles;
use crate::helpers::theme_resources::theme_resource;
use crate::helpers::error::{AppError, AppErrorAction};

pub struct App {
    clients: Vec<(Arc<client::Client>, Option<CancellationToken>)>,
    selected_client: Option<usize>,
    clients_data: Vec<client::ClientData>,

    server: Option<Arc<server::Server>>,
    server_token: Option<CancellationToken>,

    error_queue: VecDeque<AppError>,
    display_error: Option<AppError>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ExternalMessage(Arc<Mutex<mpsc::UnboundedReceiver<Message>>>, Box<Message>),
    ExternalMessageFailed(Arc<Mutex<mpsc::UnboundedReceiver<Message>>>),

    ServerConnected(Arc<server::Server>),
    ServerClientDisconnected(SocketAddr),
    ServerDisconnected,

    ClientCreated(Arc<client::Client>),
    ClientRemoved(String),
    ClientSelected(Option<usize>),
    ClientConnectionRequested(uuid::Uuid, String, String),
    ClientDisconnected(uuid::Uuid),
    ClientShouldFlush(uuid::Uuid),
    ClientHasFlushed(client::ClientData),

    Error(AppError),
    ErrorDismissed,
    NoOp,
}

static RESOURCES: OnceLock<HashMap<String, styles::ResourceValue>> = OnceLock::new();
static MESSENGER: OnceLock<mpsc::UnboundedSender<Message>> = OnceLock::new();

impl Message {
    pub fn err(title: impl Into<String>, message: impl Into<String>) -> Message {
        Message::Error(AppError::err(title, message))
    }

    pub fn log(title: impl Into<String>, message: impl Into<String>) -> Message {
        Message::Error(AppError::log(title, message))
    }
}

impl App {
    pub fn get_resources() -> &'static HashMap<String, styles::ResourceValue> {
        RESOURCES.get_or_init(|| {
            let styles: [&str; _] = [
                "TabViewItemHeaderBackgroundSelected",
                "AccentFillColorTertiaryBrush",
                "TextOnAccentFillColorPrimaryBrush",
                "CardBackgroundFillColorSecondaryBrush",
                "SolidBackgroundFillColorBaseAltBrush",
                "LayerOnMicaBaseAltFillColorDefaultBrush",
                "CardStrokeColorDefaultBrush",
            ];

            styles.iter()
                .filter_map(|key| {
                    theme_resource(key, WindowTheme::System)
                        .ok()
                        .flatten()
                        .and_then(|resource| Some((key.to_string(), resource.into())))
                })
                .collect()
        })
    }

    pub fn get_resource(key: &str) -> Option<&styles::ResourceValue> {
        Self::get_resources().get(&key.to_string())
    }

    pub fn get_brush(key: &str) -> Option<Brush> {
        Self::get_resource(key)
            .and_then(|res| match res {
                styles::ResourceValue::Color(color) => Some(color.to_owned().into()),
                _ => None,
            })
    }

    pub fn get_thickness(key: &str) -> Option<Thickness> {
        Self::get_resource(key)
            .and_then(|res| match res {
                styles::ResourceValue::Thickness(thickness) => Some(thickness.to_owned().into()),
                _ => None,
            })
    }

    pub fn get_radius(key: &str) -> Option<CornerRadius> {
        Self::get_resource(key)
            .and_then(|res| match res {
                styles::ResourceValue::CornerRadius(radius) => Some(radius.to_owned().into()),
                _ => None,
            })
    }

    pub fn get_tx() -> Option<mpsc::UnboundedSender<Message>> {
        MESSENGER.get().and_then(|tx| Some(tx.clone()))
    }

    pub fn tx(msg: Message) -> Option<()> {
        MESSENGER.get()
            .and_then(|tx| tx.clone().send(msg).ok())
    }
}

async fn recv_message(rx: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>) -> Message {
    if let Some(msg) = { rx.lock().await.recv().await } {
        Message::ExternalMessage(rx, msg.into())
    }
    else {
        Message::ExternalMessageFailed(rx)
    }
}

impl HasCancelMessage for App {
    fn cancel_message() -> <Self as Component>::Message {
        Message::NoOp
    }
}

impl Component for App {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<Message>();

        let _ = MESSENGER.set(tx);

        context.spawn_future(recv_message(Arc::new(rx.into())));

        App {
            clients: vec![],
            clients_data: vec![],
            selected_client: None,
            server: None,
            server_token: None,
            error_queue: VecDeque::new(),
            display_error: None,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::ExternalMessage(rx, msg) => {
                let _ = context.sender().send(*msg);

                context.spawn_future(recv_message(rx));
            },
            Message::ExternalMessageFailed(rx) => {
                context.spawn_future(recv_message(rx));
            },

            Message::ServerConnected(ws) => {
                self.server = Some(ws.clone());

                self.server_token = Some(context.spawn_future(ws.listen()));
            },
            Message::ServerClientDisconnected(_addr) => {},
            Message::ServerDisconnected => {
                if let Some(token) = self.server_token.take() {
                    token.cancel();
                }

                if let Some(server) = self.server.take() {
                    context.spawn_future(server.shutdown());
                }
            },

            Message::ClientCreated(client) => {
                self.clients.push((client.clone(), None));

                let _ = context.sender().send(Message::ClientShouldFlush(client.id));
            },
            Message::ClientSelected(index) => {
                self.selected_client = index;
            },
            Message::ClientConnectionRequested(id, url, nickname) => {
                self.clients.iter_mut()
                    .find_map(|elem| if elem.0.id == id { Some(elem) } else { None })
                    .and_then(|client_info| {
                        if let (client, None) = client_info {
                            let copy = client.clone();

                            let token = context.spawn_future_error(
                                async move {
                                    let result = copy.run(&url, &nickname).await;

                                    if result.is_err() {
                                        copy.disconnect().await;
                                        App::tx(Message::ClientShouldFlush(id));
                                    }

                                    Ok(result?)
                                }
                            );

                            *client_info = (client.clone(), Some(token));

                            let _ = context.sender().send(Message::ClientShouldFlush(id));

                            Some(())
                        }
                        else { None }
                    });

            },
            Message::ClientDisconnected(id) => {
                self.clients.iter_mut()
                    .find_map(|elem| if elem.0.id == id { Some(elem) } else { None })
                    .and_then(|client_info| {
                        if let (client, Some(token)) = client_info {
                            token.cancel();

                            *client_info = (client.clone(), None);

                            Some(())
                        }
                        else { None }
                    });
            },
            Message::ClientRemoved(str_id) => {
                self.clients.retain(|(client, token)| {
                    if client.id.to_string( ) == str_id {
                        token.as_ref().map(|token| token.cancel());

                        false
                    }
                    else { true }
                });

                self.clients_data.retain(|client| client.id.to_string() != str_id);
            },
            Message::ClientShouldFlush(id) => {
                self.clients.iter()
                    .find_map(|(client, _)| if client.id == id { Some(client) } else { None })
                    .and_then(|client| {
                        let client = client.clone();

                        context.spawn_future(
                            async move {
                                Message::ClientHasFlushed(client.get_data().await)
                            }
                        );

                        Some(())
                    });
            },
            Message::ClientHasFlushed(updated) => {
                self.selected_client = self.clients_data.iter_mut()
                    .find_map(|client| if client.id == updated.id { Some(client) } else { None })
                    .and_then(|client| { *client = updated.clone(); self.selected_client })
                    .or_else(|| {
                        self.clients_data.push(updated);
                        Some(self.clients_data.len() - 1)
                    });
            },

            Message::Error(err) => {
                match err.action {
                    AppErrorAction::Log => {
                        eprintln!("[LOG] {err:?}");
                    },
                    AppErrorAction::Display => {
                        if self.display_error.is_none() {
                            self.display_error = Some(err);
                        }
                        else {
                            self.error_queue.push_back(err);
                        }
                    },
                    AppErrorAction::Crash => {
                        panic!("{err:?}");
                    },
                }
            },
            Message::ErrorDismissed => {
                self.display_error = self.error_queue.pop_front();
            },
            Message::NoOp => {},
        }

    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::component::<MainWindow>({
            main_window::Input {
                selected_client: self.selected_client,
                clients_data: self.clients_data.clone(),
                display_error: self.display_error.clone(),
                server_is_running: self.server.is_some(),
            }
        })
    }
}
