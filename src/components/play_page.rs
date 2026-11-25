use windows_reactor::*;

use crate::components::{app, client_page, client_page::ClientPage};
use crate::network::client;

pub struct PlayPage {
}

#[derive(Clone, PartialEq)]
pub struct Input {
    pub clients: Vec<client::ClientData>,
    pub selected_client: Option<usize>,
}

#[derive(Clone)]
pub enum Message {
    CreateNewClient,
    RemoveClient(Option<String>),
    SelectionChanged(Option<usize>),
    NoOp,
}

fn keyed_clients(clients: &Vec<client::ClientData>) -> Vec<KeyedView> {
    clients.iter()
        .cloned()
        .map(|client| KeyedView::new(
            client.id.to_string(),
            View::component::<ClientPage>(
                client_page::Input {
                    data: client,
                }
            )
        ))
        .collect()
}

impl Component for PlayPage {
    type Input = Input;
    type Message = Message;


    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        PlayPage { }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            Message::CreateNewClient => {
                match client::Client::new() {
                    Ok(client) => app::App::tx(app::Message::ClientCreated(client)),
                    Err(err) => app::App::tx(err.into()),
                };
            },
            Message::RemoveClient(Some(id)) => {
                app::App::tx(app::Message::ClientRemoved(id));
            },
            Message::SelectionChanged(index) => {
                app::App::tx(app::Message::ClientSelected(index));
            },
            Message::NoOp => {},
            _ => {},
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let clients = keyed_clients(&input.clients);
        let selected_client = input.selected_client;

        TabView::new()
            .vertical_alignment(VerticalAlignment::Stretch)
            .margin(Thickness::new(0.0, 8.0, 0.0, 0.0))
            .selected_index(selected_client)
            .on_add_tab_button_click(context.message(Message::CreateNewClient))
            .on_selection_changed(context.callback(Message::SelectionChanged))
            .on_close_requested(context.callback(move |id: Key| {
                let id = id.as_str().and_then(|it| Some(it.to_string()));

                Message::RemoveClient(id)
            }))
            .keyed_tab_items(clients)
            .tooltip("")
            .into()
    }
}
