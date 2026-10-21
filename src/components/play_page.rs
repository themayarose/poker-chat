use windows_reactor::*;

use crate::components::{main_window, client_page, client_page::ClientPage};
use crate::network::client;

pub struct PlayPage {
}

#[derive(Clone)]
pub struct Input {
    pub window_tx: main_window::Sender,
    pub clients: Vec<client::ClientData>,
    pub selected_client: Option<usize>,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.clients == other.clients &&
        self.selected_client == other.selected_client
    }
}

#[derive(Clone)]
pub enum Message {
    CreateNewClient(main_window::Sender),
    RemoveClient(main_window::Sender, String),
    SelectionChanged(main_window::Sender, Option<usize>),
    NoOp,
}

fn keyed_clients(tx: &main_window::Sender, clients: &Vec<client::ClientData>) -> Vec<KeyedView> {
    clients.iter()
        .cloned()
        .map(|client| KeyedView::new(
            client.id.to_string(),
            View::component::<ClientPage>(
                client_page::Input {
                    data: client,
                    window_tx: tx.clone(),
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
            Message::CreateNewClient(tx) => {
                let _ = match client::Client::new() {
                    Ok(client) => tx.send(main_window::Message::ClientCreated(client)),
                    Err(err) => tx.send(err.into()),
                };
            },
            Message::RemoveClient(tx, id) => {
                let _ = tx.send(main_window::Message::ClientRemoved(id));
            },
            Message::SelectionChanged(tx, index) => {
                let _ = tx.send(main_window::Message::ClientSelected(index));
            },
            Message::NoOp => {},
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let clients = keyed_clients(&input.window_tx, &input.clients);
        let selected_client = input.selected_client;

        let tx1 = input.window_tx.clone();
        let tx2 = input.window_tx.clone();

        TabView::new()
            .vertical_alignment(VerticalAlignment::Stretch)
            .margin(Thickness::new(0.0, 8.0, 0.0, 0.0))
            .selected_index(selected_client)
            .on_add_tab_button_click(context.message(Message::CreateNewClient(input.window_tx.clone())))
            .on_selection_changed(context.callback(move |index| Message::SelectionChanged(tx1.clone(), index)))
            .on_close_requested(context.callback(move |id| Message::RemoveClient(tx2.clone(), id)))
            .collection_slot(TabViewSlot::TabItems, clients)
    }
}
