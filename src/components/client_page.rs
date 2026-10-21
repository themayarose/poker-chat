use windows_reactor::*;

use tokio::sync::mpsc;

use crate::components::main_window;
use crate::network::{client, protocol};
use crate::helpers::typography::*;
use crate::helpers::styles;

pub struct ClientPage {
    pub id: uuid::Uuid,
    pub nickname: String,
    pub server_url: String,
    pub show_banner: bool,
    pub connection_tx: Option<mpsc::UnboundedSender<protocol::ClientMessage>>,
    pub window_tx: mpsc::UnboundedSender<main_window::Message>,
}

#[derive(Clone)]
pub enum Message {
    PreConnectionNicknameChanged(String),
    PreConnectionUrlChanged(String),
    RequestConnection(main_window::Sender),
    SendText,
    Quit,
    Err(main_window::Error),
    NoOp,
}

// impl From<Box<dyn error::Error>> for Message {
//     fn from(value: Box<dyn error::Error>) -> Self {
//         Message::Err(value.into())
//     }
// }

// impl HasCancelMessage for ClientPage {
//     fn cancel_message() -> <Self as Component>::Message {
//        Message::NoOp
//     }
// }

#[derive(Clone)]
pub struct Input {
    pub window_tx: main_window::Sender,
    pub data: client::ClientData,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}


// impl Into<View> for DisplayMessage {
//     fn into(self) -> View {
//        match &self {
//            DisplayMessage::Text(_, nickname, content) => {
//            },
//            _ => {
//                List
//            }
//        }
//     }
// }

impl ClientPage {
    pub fn get_content(&self, input: &<Self as Component>::Input, context: &mut ViewContext<Self>) -> View {
        let url = &self.server_url.clone();

        let text = input.data.display.iter()
            .map(|m| format!("{m:?}"))
            .fold("".to_string(), |acc, m| acc + "\n" + m.as_str())
        ;


        let connect = StackPanel::new()
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .orientation(Orientation::Vertical)
            .spacing(8.0)
            .children([
                title("Play game")
                    .margin(Thickness::new(0.0, 0.0, 0.0, 8.0))
                    .into(),
                body("Nickname:")
                    .into(),
                TextBox::new()
                    .vertical_alignment(VerticalAlignment::Top)
                    .text(self.nickname.clone())
                    .is_enabled(!input.data.connected)
                    .on_text_changed(context.callback(Message::PreConnectionNicknameChanged))
                    .into(),
                body("URL:")
                    .into(),
                TextBox::new()
                    .vertical_alignment(VerticalAlignment::Top)
                    .text(url)
                    .is_enabled(!input.data.connected)
                    .on_text_changed(context.callback(Message::PreConnectionUrlChanged))
                    .into(),
                Button::new()
                    .on_click(context.message(Message::RequestConnection(input.window_tx.clone())))
                    .is_enabled(!input.data.connected)
                    .content("Connect")
                    .into(),
                Button::new()
                    .on_click(context.message(Message::SendText))
                    .is_enabled(input.data.connected)
                    .content("Send text")
                    .into(),
                Button::new()
                    .on_click(context.message(Message::Quit))
                    .is_enabled(input.data.connected)
                    .content("Disconnect")
                    .into(),
                TextBlock::new()
                    .text(text)
                    .text_wrapping(TextWrapping::WrapWholeWords)
                    .into(),
            ] as [View; _]);

        connect
    }
}

impl Component for ClientPage {
    type Message = Message;
    type Input = Input;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        ClientPage {
            id: input.data.id,
            nickname: input.data.nickname.clone(),
            server_url: input.data.url.clone().unwrap_or("".into()),
            show_banner: false,
            connection_tx: input.data.tx.clone(),
            window_tx: input.window_tx.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
       self.connection_tx = input.data.tx.clone();
       self.window_tx = input.window_tx.clone();
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            Message::PreConnectionUrlChanged(url) => {
                self.server_url = url;
            },
            Message::PreConnectionNicknameChanged(nickname) => {
                self.nickname = nickname;
            },
            Message::RequestConnection(tx) => {
                let tx = tx.clone();
                let id = self.id;
                let nickname = self.nickname.clone();
                let url = self.server_url.clone();

                let _ = tx.send(main_window::Message::ClientConnectionRequested(id, url, nickname));
            },
            Message::SendText => {
                if let Some(tx) = self.connection_tx.clone() {
                    let _ = tx.send(protocol::ClientMessage::SendMessage("hello world!".into()));
                }
            },
            Message::Quit => {
                if let Some(tx) = self.connection_tx.clone() {
                    let _ = tx.send(protocol::ClientMessage::Quit);
                    let _ = self.window_tx.send(main_window::Message::ClientDisconnected(self.id));
                }
            },
            Message::Err(err) => {
                let _ = self.window_tx.send(err.into());
            },
            Message::NoOp => {}
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let nickname = &input.data.nickname;
        let url = input.data.url.clone().unwrap_or("no url".to_string());

        let title = if input.data.connected { format!("{nickname}@{url}") }
            else { "(disconnected)".into() };

        let tag = input.data.id.to_string();

        let mut border = Border::new()
            .vertical_alignment(VerticalAlignment::Stretch);

        let styles = context.use_context(&main_window::STYLES);

        if let Some(styles::ResourceValue::Color(tabview_bg)) = styles.get("TabViewItemHeaderBackgroundSelected") {
            border = border.background(tabview_bg.clone());
        }

        TabViewItem::new()
            .vertical_alignment(VerticalAlignment::Stretch)
            .header(title)
            .tag(tag)
            .content(
                border
                    .padding(16.0)
                    .content(self.get_content(input, context))
            )
    }
}
