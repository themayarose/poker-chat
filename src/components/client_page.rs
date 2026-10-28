use windows_reactor::*;

use tokio::sync::mpsc;

use crate::components::main_window;
use crate::network::{client, protocol};
use crate::helpers::typography::*;
use crate::helpers::styles;

pub struct ClientPage {
    id: uuid::Uuid,
    nickname: String,
    server_url: String,
    connection_tx: Option<mpsc::UnboundedSender<protocol::ClientMessage>>,
    window_tx: mpsc::UnboundedSender<main_window::Message>,
    message: String,
}

#[derive(Clone)]
pub enum Message {
    PreConnectionNicknameChanged(String),
    PreConnectionUrlChanged(String),
    RequestConnection(main_window::Sender),
    TextChanged(String),
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


impl Into<View> for client::DisplayMessage {
    fn into(self) -> View {
       match &self {
           client::DisplayMessage::Text(_, nickname, content) => {
               ListViewItem::new()
                   .content(format!("<{nickname}> {content}"))
           },
           _ => {
               ListViewItem::new()
                   .content(format!("{:?}", self))
           }
       }
    }
}



impl ClientPage {
    pub fn get_content(&self, input: &<Self as Component>::Input, context: &mut ViewContext<Self>) -> View {
        let url = &self.server_url.clone();

        let connect = StackPanel::new()
            .grid_row(0)
            .grid_column(0)
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
                    .on_click(context.message(Message::Quit))
                    .is_enabled(input.data.connected)
                    .content("Disconnect")
                    .into(),
            ] as [View; _]);

        connect
    }

    fn layout(&self, input: &<Self as Component>::Input, context: &mut ViewContext<Self>) -> View {
        Grid::new()
            .columns([GridLength::Star(2.0), GridLength::Star(1.0)])
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .children([
                self.game(input, context),
                self.chat(input, context),
            ] as [View; _])
    }

    fn game(&self, input: &<Self as Component>::Input, context: &mut ViewContext<Self>) -> View {
        self.get_content(input, context)
        // Grid::new()
        //     .grid_column(0)
        //     .children([
        //     ] as [View; _])
    }

    fn chat(&self, input: &<Self as Component>::Input, context: &mut ViewContext<Self>) -> View {
        let messages: Vec<KeyedView> = input.data.display.iter()
            .cloned()
            .enumerate()
            .map(|(i, m)| KeyedView::new(i, m))
            .collect();

        Grid::new()
            .grid_column(1)
            .rows([GridLength::STAR, GridLength::Auto])
            .columns([GridLength::STAR, GridLength::Auto])
            .row_spacing(4.0)
            .column_spacing(2.0)
            .children([
                ListView::new()
                    .grid_row(0)
                    .grid_column_span(2)
                    .selection_mode(ListViewSelectionMode::None)
                    .collection_slot(ListViewSlot::Items, messages),
                TextBox::new()
                    .grid_row(1)
                    .grid_column(0)
                    .text(self.message.clone())
                    .on_text_changed(context.callback(Message::TextChanged))
                    .accepts_return(true)
                    .into(),
                Button::new()
                    .grid_row(1)
                    .grid_column(1)
                    .on_click(context.message(Message::SendText))
                    .content("Send"),
            ] as [View; _])
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
            connection_tx: input.data.tx.clone(),
            window_tx: input.window_tx.clone(),
            message: "".into(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
       self.connection_tx = input.data.tx.clone();
       self.window_tx = input.window_tx.clone();
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
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
            Message::TextChanged(msg) => {
                if msg.ends_with("\r") { context.sender().send(Message::SendText); }
                else { self.message = msg; }
            },
            Message::SendText => {
                if let Some(tx) = self.connection_tx.clone() {
                    let _ = tx.send(protocol::ClientMessage::SendMessage(self.message.clone()));
                }

                self.message = "".into();
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
                    .content(self.layout(input, context))
            )
    }
}
