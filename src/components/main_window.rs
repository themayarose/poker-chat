use std::rc::Rc;

use windows_reactor::*;

use crate::extensions::*;

use crate::components::app;
use crate::components::play_page::{self, PlayPage};
use crate::components::host_page::{self, HostPage};
use crate::components::settings_page::{self, SettingsPage};
use crate::network::client;
use crate::helpers::error::AppError;

pub struct MainWindow {
    is_pane_open: bool,
    selected_page: Page,
    header_message: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Input {
    pub server_is_running: bool,
    pub selected_client: Option<usize>,
    pub clients_data: Vec<client::ClientData>,
    pub display_error: Option<AppError>,
}

#[derive(Debug, Clone)]
pub enum Message {
    PaneOpenChanged,
    SelectedTagChanged(Option<String>),
    ErrorDismissed,
    NoOp,
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

impl MainWindow {
    fn make_navigation_item(&self, display_name: impl Into<String>, tag: Page, icon: impl Into<Icon>) -> NavigationViewItem {
        let is_selected = |tag: Page| self.selected_page == tag;
        let tag_str: String = tag.into();

        NavigationViewItem::new()
            .is_selected(is_selected(tag))
            .tag(tag_str)
            .icon(icon)
            .content(display_name.into())
    }

    fn get_navigation_items(&self) -> Vec<View> {
        let play_icon = Icon::font("\u{E7FC}");
        let host_icon = Icon::font("\u{F191}");

        vec![
            self.make_navigation_item("Play", Page::Play, play_icon).into(),
            self.make_navigation_item("Host", Page::Host, host_icon).into(),
        ]
    }

    fn get_error_dialog(&self, input: &<Self as Component>::Input, context: &mut ViewContext<Self>) -> ContentDialog {
        let error = input.display_error.clone();

        ContentDialog::new()
            .is_open(error.is_some())
            .primary_button_text("OK")
            .is_secondary_button_enabled(false)
            .title(error.clone().and_then(|err| Some(err.title.clone())).unwrap_or("Error not found".into()))
            .on_closed(context.callback(|_| Message::ErrorDismissed))
            .content(error.and_then(|err| Some(err.message.clone())).unwrap_or("Message not found".into()))
    }

    fn get_navigation_view(&self, input: &<Self as Component>::Input, context: &mut ViewContext<Self>, view: View) -> View {
        NavigationView::new()
            .vertical_alignment(VerticalAlignment::Stretch)
            .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed)
            .is_pane_toggle_button_visible(false)
            .is_settings_visible(true)
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .is_pane_open(self.is_pane_open)
            .menu_items(self.get_navigation_items())
            .content(view)
            .on_selected_tag_changed(context.callback(|tag: Option<Rc<str>>| Message::SelectedTagChanged(tag.map(|tag| tag.to_string()))))
            .content_dialog(self.get_error_dialog(input, context).into())
            .into()
    }

    fn get_titlebar(&self, context: &mut ViewContext<Self>) -> View {
        context.titlebar("PokerChat")
            .is_pane_toggle_button_visible(true)
            .is_back_button_visible(false)
            .on_pane_toggle_requested(context.message(Message::PaneOpenChanged))
            .subtitle("Simple poker server/client")
            .right_header(View::from(self.header_message.clone()))
            .content(View::from(""))
            .into()
    }
}

impl HasCancelMessage for MainWindow {
    fn cancel_message() -> <Self as Component>::Message {
       Message::NoOp
    }
}

impl Component for MainWindow {
    type Input = Input;
    type Message = Message;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        match client::Client::new() {
            Ok(client) => app::App::tx(app::Message::ClientCreated(client)),
            Err(e) => app::App::tx(e.into()),
        };

        MainWindow {
            is_pane_open: false,
            selected_page: Page::Play,
            header_message: "Server is off".into(),
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            Message::PaneOpenChanged => self.is_pane_open = !self.is_pane_open,
            Message::SelectedTagChanged(Some(tag)) => {
                let page: Page = tag.try_into()
                    .expect("Unknown page requested");

                self.selected_page = page;
            },
            Message::SelectedTagChanged(None) => {},
            Message::ErrorDismissed => {
                app::App::tx(app::Message::ErrorDismissed);
            },
            Message::NoOp => {},
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.header_message = {
            if input.server_is_running { "Server is ON".into() }
            else { "Server is off".into() }
        };
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let titlebar = self.get_titlebar(context);

        let frame = match self.selected_page {
            Page::Play => {
                View::component::<PlayPage>(play_page::Input {
                    clients: input.clients_data.clone(),
                    selected_client: input.selected_client,
                })
            },
            Page::Host => {
                View::component::<HostPage>(host_page::Input {
                    server_is_running: input.server_is_running,
                })
            },
            Page::Settings => {
                View::component::<SettingsPage>(settings_page::Input {
                })
            },
        };

        let navigation_view = self.get_navigation_view(input, context, frame);

        context.with_titlebar(WindowBackdrop::Mica, titlebar, navigation_view)
    }
}
