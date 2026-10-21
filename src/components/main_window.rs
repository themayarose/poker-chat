use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::net::SocketAddr;
use std::error;
use std::fmt;
use std::sync::LazyLock;

use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite;

use windows_reactor::*;

use tokio::sync::mpsc;

use crate::extensions::component_context_ext::HasCancelMessage;
use crate::extensions::*;

use crate::components::play_page::{self, PlayPage};
use crate::components::host_page::{self, HostPage};
use crate::components::settings_page::{self, SettingsPage};
use crate::network::{server, client};
use crate::helpers::theme_resources::theme_resource;
use crate::helpers::styles;

pub type Sender = mpsc::UnboundedSender<Message>;

pub struct MainWindow {
    pub tx: Sender,
    pub is_pane_open: bool,
    pub selected_page: Page,
    pub header_message: String,
    pub clients: Vec<(Arc<client::Client>, Option<ComponentTask>)>,
    pub clients_data: Vec<client::ClientData>,
    pub selected_client: Option<usize>,
    pub server_url: String,
    pub server: Option<Arc<server::Server>>,
    pub server_task: Option<ComponentTask>,
    pub error_queue: VecDeque<Error>,
    pub display_error: Option<Error>,
    pub styles: HashMap<String, styles::ResourceValue>,
}

#[derive(Clone)]
pub enum Message {
    PaneOpenChanged,
    SelectedTagChanged(Option<String>),

    ServerConnected(Arc<server::Server>),
    ServerClientDisconnected(SocketAddr),
    ServerUrlChanged(String),
    ServerDisconnected,

    ClientCreated(Arc<client::Client>),
    ClientSelected(Option<usize>),
    ClientConnectionRequested(uuid::Uuid, String, String),
    ClientDisconnected(uuid::Uuid),
    ClientRemoved(String),
    FlushClient(uuid::Uuid),
    ClientFlushed(client::ClientData),

    ExternalMessage(Arc<Mutex<mpsc::UnboundedReceiver<Message>>>, Box<Message>),
    ExternalMessageFailed(Arc<Mutex<mpsc::UnboundedReceiver<Message>>>),

    Error(Error),
    ErrorDismissed,
    NoOp,
}

impl From<Error> for Message {
    fn from(err: Error) -> Self {
        Message::Error(err)
    }
}

impl From<Box<dyn error::Error>> for Message {
    fn from(err: Box<dyn error::Error>) -> Self {
        <Box<dyn error::Error> as Into<Error>>::into(err).into()
    }
}

impl Message {
    pub fn err(title: impl Into<String>, message: impl Into<String>) -> Message {
        Message::Error(Error::err(title, message))
    }

    pub fn log(title: impl Into<String>, message: impl Into<String>) -> Message {
        Message::Error(Error::log(title, message))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Play,
    Host,
    Settings,
}


impl TryFrom<String> for Page {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "play" { Ok(Page::Play) }
        else if value == "host" { Ok(Page::Host) }
        else if value == "Settings" { Ok(Page::Settings) }
        else { Err(()) }
    }
}

impl Into<String> for Page {
    fn into(self) -> String {
        match self {
            Page::Play => "play".to_string(),
            Page::Host => "host".to_string(),
            Page::Settings => "Settings".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub title: String,
    pub message: String,
    pub action: ErrorAction,
}

#[derive(Debug, Clone)]
pub enum ErrorAction {
    Log,
    Display,
    Crash,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.title, self.message)
    }
}

impl error::Error for Error {}

impl From<Box<dyn error::Error>> for Error {
    fn from(err: Box<dyn error::Error>) -> Self {
        match err.downcast::<tungstenite::Error>() {
            Ok(normal) if matches!(*normal, tungstenite::Error::ConnectionClosed) => Error::log("Client", "Disconnected normally"),
            Ok(tng_err) => Error::err("Error", format!("{tng_err}")),
            Err(other_err) => Error::err("Error", format!("{other_err}")),
        }
    }
}

impl Error {
    pub fn err(title: impl Into<String>, message: impl Into<String>) -> Error {
        Error {
            title: title.into().clone(),
            message: message.into().clone(),
            action: ErrorAction::Display,
        }
    }

    pub fn log(title: impl Into<String>, message: impl Into<String>) -> Error {
        Error {
            title: title.into().clone(),
            message: message.into().clone(),
            action: ErrorAction::Log,
        }
    }
}

impl MainWindow {
    fn make_navigation_item(&self, display_name: impl Into<String>, tag: Page, icon: impl Into<View>) -> (String, View) {
        let item_slot = |(a, b): (NavigationViewItemSlot, View)| SlotView::new(a, b);
        let is_selected = |tag: Page| self.selected_page == tag;

        (tag.into(), NavigationViewItem::new()
            .is_selected(is_selected(tag))
            .tag(tag)
            .slots([
                (NavigationViewItemSlot::Icon, icon.into()),
                (NavigationViewItemSlot::Content, display_name.into().into()),
            ].map(item_slot)),
        )
    }

    fn get_navigation_items(&self) -> Vec<(String, View)> {
        let play_icon = FontIcon::new().glyph("\u{E7FC}");
        let host_icon = FontIcon::new().glyph("\u{F191}");

        vec![
            self.make_navigation_item("Play", Page::Play, play_icon),
            self.make_navigation_item("Host", Page::Host, host_icon),
        ]
    }

    fn get_error_dialog(&self, context: &mut ViewContext<Self>) -> View {
        let error = self.display_error.clone();

        ContentDialog::new()
            .is_open(error.is_some())
            .primary_button_text("OK")
            .is_secondary_button_enabled(false)
            .title(error.clone().and_then(|err| Some(err.title.clone())).unwrap_or("Error not found".into()))
            .on_closed(context.callback(|_| Message::ErrorDismissed))
            .content(error.and_then(|err| Some(err.message.clone())).unwrap_or("Message not found".into()))
            .into()
    }

    fn get_navigation_view(&self, context: &mut ViewContext<Self>, view: View) -> View {
        let panel = Grid::new()
            .vertical_alignment(VerticalAlignment::Stretch)
            .rows([GridLength::Pixel(0.0), GridLength::STAR])
            .children([
                self.get_error_dialog(context),
                Border::new().grid_row(1).content(view),
            ]);

        NavigationView::new()
            .vertical_alignment(VerticalAlignment::Stretch)
            .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed)
            .is_pane_toggle_button_visible(false)
            .is_settings_visible(true)
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .is_pane_open(self.is_pane_open)
            .on_selected_tag_changed(context.callback(Message::SelectedTagChanged))
            .slots([
                SlotView::collection(NavigationViewSlot::MenuItems, self.get_navigation_items()),
                SlotView::new(NavigationViewSlot::Content, panel),
            ])
    }

    fn get_titlebar(&self, context: &mut ViewContext<Self>) -> View {
        let item_slot = |(a, b): (TitleBarSlot, View)| SlotView::new(a, b);

        context.titlebar("PokerChat")
            .is_pane_toggle_button_visible(Some(true))
            .is_back_button_visible(Some(false))
            .on_pane_toggle_requested(context.message(Message::PaneOpenChanged))
            .subtitle("Simple poker server/client")
            .slots([
                (TitleBarSlot::RightHeader, self.header_message.clone().into()),
                (TitleBarSlot::Content, self.server_url.clone().into()),
            ].map(item_slot))
    }

    fn get_resources() -> HashMap<String, styles::ResourceValue> {
        let styles: [&str; _] = [
            "TabViewItemHeaderBackgroundSelected",
        ];

        styles.iter()
            .filter_map(|key| {
                theme_resource(key, WindowTheme::System)
                    .ok()
                    .flatten()
                    .and_then(|resource| Some((key.to_string(), resource.into())))
            })
            .collect()
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

pub static STYLES: LazyLock<Context<HashMap<String, styles::ResourceValue>>> =
    LazyLock::new(|| Context::new(HashMap::new()));

impl HasCancelMessage for MainWindow {
    fn cancel_message() -> <Self as Component>::Message {
       Message::NoOp
    }
}

impl Component for MainWindow {
    type Input = ();
    type Message = Message;

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<Message>();

        context.spawn_future(recv_message(Arc::new(Mutex::new(rx))));

        match client::Client::new() {
            Ok(client) => context.sender().send(Message::ClientCreated(client)),
            Err(e) => context.sender().send(e.into()),
        };

        MainWindow {
            tx: tx,
            is_pane_open: false,
            selected_page: Page::Play,
            header_message: "Disconnected".into(),
            clients: vec![],
            clients_data: vec![],
            selected_client: None,
            server: None,
            server_url: "127.0.0.1:8080".into(),
            server_task: None,
            error_queue: VecDeque::new(),
            display_error: None,
            styles: Self::get_resources(),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::PaneOpenChanged => self.is_pane_open = !self.is_pane_open,
            Message::SelectedTagChanged(Some(tag)) => {
                let page: Page = tag.try_into()
                    .expect("Unknown page requested");

                self.selected_page = page;
            },
            Message::SelectedTagChanged(None) => {},


            Message::ServerConnected(ws) => {
                self.server = Some(ws);

                let server = self.server.clone().unwrap();
                let tx = self.tx.clone();

                self.server_task = Some(context.spawn_future(server.listen(tx)));
            },
            Message::ServerClientDisconnected(_addr) => {},
            Message::ServerUrlChanged(url) => {
                self.server_url = url;
            },
            Message::ServerDisconnected => {
                if let Some(task) = &self.server_task {
                    task.cancel();
                    self.server_task = None;
                }

                self.server = None;
            },



            Message::ClientCreated(client) => {
                self.clients.push((client.clone(), None));

                context.sender().send(Message::FlushClient(client.id));
            },
            Message::ClientSelected(index) => {
                self.selected_client = index;
            },
            Message::ClientConnectionRequested(id, url, nickname) => {
                self.clients.iter_mut()
                    .find_map(|elem| if elem.0.id == id { Some(elem) } else { None })
                    .and_then(|client_info| {
                        if let (client, None) = client_info {
                            let tx = self.tx.clone();
                            let copy = client.clone();

                            let task = context.spawn_future_error(
                                async move {
                                    copy.run(tx, &url, &nickname).await
                                }
                            );

                            *client_info = (client.clone(), Some(task));

                            context.sender().send(Message::FlushClient(id));

                            Some(())
                        }
                        else { None }
                    });

            },
            Message::ClientDisconnected(id) => {
                self.clients.iter_mut()
                    .find_map(|elem| if elem.0.id == id { Some(elem) } else { None })
                    .and_then(|client_info| {
                        if let (client, Some(task)) = client_info {
                            task.cancel();

                            *client_info = (client.clone(), None);

                            Some(())
                        }
                        else { None }
                    });
            },
            Message::ClientRemoved(str_id) => {
                self.clients.retain(|(client, task)| {
                    if client.id.to_string( ) == str_id {
                        task.as_ref().map(|task| task.cancel());

                        false
                    }
                    else { true }
                });
                self.clients_data.retain(|client| client.id.to_string() != str_id);
            },
            Message::FlushClient(id) => {
                self.clients.iter()
                    .find_map(|(client, _)| if client.id == id { Some(client) } else { None })
                    .and_then(|client| {
                        let client = client.clone();

                        context.spawn_future(
                            async move {
                                Message::ClientFlushed(client.get_data().await)
                            }
                        );

                        Some(())
                    });
            },
            Message::ClientFlushed(updated) => {
                self.selected_client = self.clients_data.iter_mut()
                    .find_map(|client| if client.id == updated.id { Some(client) } else { None })
                    .and_then(|client| { *client = updated.clone(); self.selected_client })
                    .or_else(|| {
                        self.clients_data.push(updated);
                        Some(self.clients_data.len() - 1)
                    });
            },


            Message::ExternalMessage(rx, msg) => {
                context.sender().send(*msg);

                context.spawn_future(recv_message(rx));
            },
            Message::ExternalMessageFailed(rx) => {
                context.spawn_future(recv_message(rx));
            },


            Message::Error(err) => {
                match err.action {
                    ErrorAction::Log => {
                        eprintln!("[LOG] {err:?}");
                    },
                    ErrorAction::Display => {
                        if self.display_error.is_none() {
                            self.display_error = Some(err);
                        }
                        else {
                            self.error_queue.push_back(err);
                        }
                    },
                    ErrorAction::Crash => {
                        panic!("{err:?}");
                    },
                }
            },
            Message::ErrorDismissed => {
                self.display_error = self.error_queue.pop_front();
            },
            Message::NoOp => {},
        }

        if self.server_task.is_some() {
            self.header_message = "Connected".into();
        }
        else {
            self.header_message = "Disconnected".into();
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let titlebar = self.get_titlebar(context);

        let frame = match self.selected_page {
            Page::Play => {
                View::component::<PlayPage>(play_page::Input {
                    window_tx: self.tx.clone(),
                    clients: self.clients_data.clone(),
                    selected_client: self.selected_client,
                })
            },
            Page::Host => {
                View::component::<HostPage>(host_page::Input {
                    window_tx: self.tx.clone(),
                    server: self.server.clone(),
                    server_url: self.server_url.clone(),
                })
            },
            Page::Settings => {
                View::component::<SettingsPage>(settings_page::Input {
                    window_tx: self.tx.clone(),
                })
            },
        };

        let navigation_view = self.get_navigation_view(context, frame);

        View::provide(&STYLES, self.styles.clone(),
            context.with_titlebar(WindowBackdrop::Mica, titlebar, navigation_view)
        )
    }
}
