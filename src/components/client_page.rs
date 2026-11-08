use chrono::{self, DateTime, Local};

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
    RequestConnection,
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

struct MessageContext<'a>(client::DisplayMessage, &'a mut ViewContext<ClientPage>, Option<uuid::Uuid>);

impl Into<View> for MessageContext<'_> {
    fn into(self) -> View {
        let MessageContext(message, context, id_on_server) = self;
        let styles = context.use_context(&main_window::STYLES);

        let card_bg: Option<Brush> = match styles.get("CardBackgroundFillColorSecondaryBrush") {
            Some(styles::ResourceValue::Color(color)) => Some(color.to_owned().into()),
            _ => None,
        };

        let highlight_bg: Option<Brush> = match styles.get("AccentFillColorTertiaryBrush") {
            Some(styles::ResourceValue::Color(color)) => Some(color.to_owned().into()),
            _ => None,
        };

        let highlight_fg: Option<Brush> = match styles.get("TextOnAccentFillColorPrimaryBrush") {
            Some(styles::ResourceValue::Color(color)) => Some(color.to_owned().into()),
            _ => None,
        };

        let info_bg: Option<Brush> = match styles.get("SolidBackgroundFillColorBaseAltBrush") {
            Some(styles::ResourceValue::Color(color)) => Some(color.to_owned().into()),
            _ => None,
        };

        ListViewItem::new()
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .margin(Thickness::new(0.0, 0.0, 0.0, 12.0))
            .content(
                match message {
                    client::DisplayMessage::Text(now, id, nickname, content) => {
                        let is_self = Some(id) == id_on_server;
                        let highlight_bg = if is_self { highlight_bg } else { card_bg };
                        let highlight_fg = if is_self { highlight_fg } else { None };
                        let alignment = if is_self { HorizontalAlignment::Right } else { HorizontalAlignment::Left };

                        Border::new()
                            .corner_radius(4.0)
                            .horizontal_alignment(alignment)
                            .background_optional(highlight_bg)
                            .padding(12.0)
                            .content(
                                Grid::new()
                                    .row_spacing(4.0)
                                    .column_spacing(8.0)
                                    .rows([GridLength::Auto, GridLength::Auto])
                                    .columns([GridLength::STAR, GridLength::Auto])
                                    .children([
                                        body_strong(nickname)
                                            .vertical_alignment(VerticalAlignment::Bottom)
                                            .grid_row(0)
                                            .grid_column(0)
                                            .foreground_optional(highlight_fg),
                                        caption(format!("{}", now.format("%H:%M:%S")))
                                            .vertical_alignment(VerticalAlignment::Bottom)
                                            .grid_row(0)
                                            .grid_column(1)
                                            .foreground_optional(highlight_fg),
                                        body(content)
                                            .text_wrapping(TextWrapping::WrapWholeWords)
                                            .grid_row(1)
                                            .grid_column(0)
                                            .grid_column_span(2)
                                            .foreground_optional(highlight_fg),
                                    ])
                            )
                    },
                    client::DisplayMessage::Joined(now, _, nickname) => {
                        control_message(now, info_bg, format!("{nickname} has joined the chat."))
                    },
                    client::DisplayMessage::Left(now, _, nickname) => {
                        control_message(now, info_bg, format!("{nickname} has left the chat."))
                    },
                    client::DisplayMessage::NowKnownAs(now, _, old_nickname, new_nickname) => {
                        control_message(now, info_bg, format!("{old_nickname} is now known as {new_nickname}."))
                    },
                    client::DisplayMessage::NicknameTaken(now, nickname) => {
                        control_message(now, info_bg, format!("The nickname {nickname:?} is already taken."))
                    },
                    client::DisplayMessage::Pong(now, delta) => {
                        let delta: f64 = match delta.num_microseconds() {
                            Some(delta) => (delta as f64) / 1000.0,
                            None => f64::MAX / 1000.0,
                        };

                        control_message(now, info_bg, format!("Lag: {delta}ms"))
                    },
                }
            )
    }
}

fn control_message(now: DateTime<Local>, color: Option<Brush>, text: impl Into<String>) -> View {
    Border::new()
        .corner_radius(4.0)
        .horizontal_alignment(HorizontalAlignment::Center)
        .background_optional(color)
        .padding(8.0)
        .content(StackPanel::new().orientation(Orientation::Vertical).spacing(2.0)
            .children([
                caption(format!("{}", now.format("%H:%M:%S"))).horizontal_alignment(HorizontalAlignment::Center),
                body(text).horizontal_alignment(HorizontalAlignment::Center),
            ])
        )
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
                    .on_click(context.message(Message::RequestConnection))
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
            .column_spacing(16.0)
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
            .map(|(i, m)| KeyedView::new(i, MessageContext(m, context, input.data.id_on_server)))
            .collect();

        let styles = context.use_context(&main_window::STYLES);

        let chat_bg: Option<Brush> = match styles.get("LayerOnMicaBaseAltFillColorDefaultBrush") {
            Some(styles::ResourceValue::Color(color)) => Some(color.to_owned().into()),
            _ => None,
        };

        let chat_border: Option<Brush> = match styles.get("CardStrokeColorDefaultBrush") {
            Some(styles::ResourceValue::Color(color)) => Some(color.to_owned().into()),
            _ => None,
        };


        Grid::new()
            .grid_column(1)
            .rows([GridLength::STAR, GridLength::Auto])
            .columns([GridLength::STAR, GridLength::Auto])
            .row_spacing(8.0)
            .column_spacing(8.0)
            .children([
                Border::new()
                    .grid_row(0)
                    .grid_column_span(2)
                    .background_optional(chat_bg)
                    .border_brush_optional(chat_border)
                    .border_thickness(1.0)
                    .corner_radius(4.0)
                    .padding(Thickness::new(0.0, 8.0, 0.0, 8.0))
                    .content(
                        ListView::new()
                            .selection_mode(ListViewSelectionMode::None)
                            .collection_slot(ListViewSlot::Items, messages)
                    ),
                Grid::new()
                    .grid_row(1)
                    .grid_column(0)
                    .key_accelerators(KeyAccelerators::new([
                            KeyAccelerator::new(
                                AcceleratorKey::Enter,
                                AcceleratorModifiers::None,
                                context.message(Message::SendText)
                            ),
                    ]))
                    .children([
                        TextBox::new()
                            .text(self.message.clone())
                            .is_enabled(input.data.connected)
                            .on_text_changed(context.callback(Message::TextChanged))
                            .into()
                    ] as [View; _]),
                Button::new()
                    .grid_row(1)
                    .grid_column(1)
                    .is_enabled(input.data.connected)
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

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            Message::PreConnectionUrlChanged(url) => {
                self.server_url = url;
            },
            Message::PreConnectionNicknameChanged(nickname) => {
                self.nickname = nickname;
            },
            Message::RequestConnection => {
                let id = self.id;
                let nickname = self.nickname.clone();
                let url = self.server_url.clone();

                let _ = self.window_tx.send(main_window::Message::ClientConnectionRequested(id, url, nickname));
            },
            Message::TextChanged(msg) => {
                self.message = msg;
            },
            Message::SendText => {
                if let Some(tx) = self.connection_tx.clone() {
                    if self.message.starts_with("/nick") {
                        let (_, new_nick) = self.message.split_at(5);
                        let new_nick = new_nick.trim();

                        let _ = tx.send(protocol::ClientMessage::SetNickname(new_nick.into()));
                    }
                    else if self.message.starts_with("/ping") {
                        let _ = tx.send(protocol::ClientMessage::Ping(Local::now()));
                    }
                    else {
                        let _ = tx.send(protocol::ClientMessage::SendMessage(self.message.clone()));
                    }
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
            border = border.background(tabview_bg.to_owned());
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
            .tooltip("")
    }
}
