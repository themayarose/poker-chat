use std::error::Error;
use std::rc::Rc;

use windows_reactor::*;

use crate::components::app;
use crate::extensions::component_context_ext::HasCancelMessage;
use crate::network::server;
use crate::extensions::*;

use crate::helpers::typography::*;
use crate::helpers::error::AppError;

pub struct HostPage {
    url: String,
}

#[derive(Clone, PartialEq)]
pub struct Input {
    pub server_is_running: bool,
}

#[derive(Clone)]
pub enum Message {
    ServerUrlChanged(String),
    StartServer,
    StopServer,
    Err(AppError),
    NoOp,
}

impl From<Box<dyn Error>> for Message {
    fn from(value: Box<dyn Error>) -> Self {
        Message::Err(value.into())
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

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        HostPage {
            url: "127.0.0.1:12345".into(),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message  {
            Message::ServerUrlChanged(url) => {
                self.url = url.clone();
            },
            Message::StartServer => {
                let url = self.url.clone();

                context.spawn_future_error(
                    async move {
                        let server = server::Server::new(url).await?;

                        app::App::tx(app::Message::ServerConnected(server));

                        Ok(Message::NoOp)
                    }
                );
            },
            Message::StopServer => {
                app::App::tx(app::Message::ServerDisconnected);
            },
            Message::Err(err) => {
                app::App::tx(app::Message::Error(err));
            },
            Message::NoOp => {},
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
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
                    .is_enabled(!input.server_is_running)
                    .vertical_alignment(VerticalAlignment::Top)
                    .text(self.url.clone())
                    .on_text_changed(context.callback(|it: Rc<str>| Message::ServerUrlChanged(it.to_string())))
                    .into(),
                StackPanel::new()
                    .grid_row(3)
                    .horizontal_alignment(HorizontalAlignment::Right)
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children([
                        Button::new()
                            .is_enabled(!input.server_is_running)
                            .on_click(context.message(Message::StartServer))
                            .content("Start server".to_string())
                            .into(),
                        Button::new()
                            .grid_row(3)
                            .is_enabled(input.server_is_running)
                            .on_click(context.message(Message::StopServer))
                            .content("Stop server".to_string())
                            .into(),
                    ] as [View; _])
                    .into()
            ] as [View; _])
            .into()
    }
}

