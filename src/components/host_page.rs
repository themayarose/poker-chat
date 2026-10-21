use std::sync::Arc;
use std::error::Error;

use windows_reactor::*;

use crate::components::main_window;
use crate::extensions::component_context_ext::HasCancelMessage;
use crate::network::server;
use crate::extensions::*;

use crate::helpers::typography::*;

pub struct HostPage {
    window_tx: main_window::Sender,
    url: String,
}

#[derive(Clone)]
pub struct Input {
    pub window_tx: main_window::Sender,
    pub server_url: String,
    pub server: Option<Arc<server::Server>>,
}

#[derive(Clone)]
pub enum Message {
    ServerUrlChanged(String),
    StartServer,
    StopServer,
    Err(main_window::Error),
    NoOp,
}

impl From<Box<dyn Error>> for Message {
    fn from(value: Box<dyn Error>) -> Self {
        Message::Err(value.into())
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        (self.server == other.server) &&
        (self.server_url == other.server_url)
    }
}

impl HostPage {
}

impl HasCancelMessage for HostPage {
    fn cancel_message() -> <Self as Component>::Message {
        Message::NoOp
    }
}

impl Component for HostPage {
    type Input = Input;
    type Message = Message;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        HostPage {
            window_tx: input.window_tx.clone(),
            url: input.server_url.clone(),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message  {
            Message::ServerUrlChanged(url) => {
                self.url = url.clone();

                let _ = self.window_tx.send(main_window::Message::ServerUrlChanged(url));
            },
            Message::StartServer => {
                let tx = self.window_tx.clone();
                let url = self.url.clone();

                context.spawn_future_error(
                    async move {
                        let server = server::Server::new(url).await?;

                        let _ = tx.send(main_window::Message::ServerConnected(server));

                        Ok(Message::NoOp)
                    }
                );
            },
            Message::StopServer => {
                let _ = self.window_tx.send(main_window::Message::ServerDisconnected);
            },
            Message::Err(err) => {
                let _ = self.window_tx.send(main_window::Message::Error(err));
            },
            Message::NoOp => {},
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.window_tx = input.window_tx.clone();

        if self.url != input.server_url {
            self.url = input.server_url.clone();
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let is_url_editable = input.server.is_none();

        Grid::new()
            .margin(16.0)
            .rows([GridLength::Auto, GridLength::Auto, GridLength::Auto, GridLength::Auto, GridLength::STAR])
            .row_spacing(8.0)
            .children([
                title("Host game")
                    .margin(Thickness::new(0.0, 0.0, 0.0, 8.0))
                    .grid_row(0)
                    .into(),
                body("URL:")
                    .grid_row(1)
                    .into(),
                TextBox::new()
                    .grid_row(2)
                    .is_enabled(is_url_editable)
                    .vertical_alignment(VerticalAlignment::Top)
                    .text(self.url.clone())
                    .on_text_changed(context.callback(Message::ServerUrlChanged))
                    .into(),
                StackPanel::new()
                    .grid_row(3)
                    .horizontal_alignment(HorizontalAlignment::Right)
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children([
                        Button::new()
                            .is_enabled(is_url_editable)
                            .on_click(context.message(Message::StartServer))
                            .content("Start server".to_string())
                            .into(),
                        Button::new()
                            .grid_row(3)
                            .is_enabled(!is_url_editable)
                            .on_click(context.message(Message::StopServer))
                            .content("Stop server".to_string())
                            .into(),
                    ] as [View; _])
            ] as [View; _])
    }
}

