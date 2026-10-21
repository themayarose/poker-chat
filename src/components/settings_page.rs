use windows_reactor::*;

use crate::components::main_window;
use crate::helpers::typography::*;

pub struct SettingsPage {
}

#[derive(Clone)]
pub struct Input {
    pub window_tx: main_window::Sender,
}

impl PartialEq for Input {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Component for SettingsPage {
    type Input = Input;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        SettingsPage { }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Grid::new()
            .margin(16.0)
            .row_spacing(8.0)
            .rows([GridLength::Auto, GridLength::STAR])
            .children([
                title("Settings")
                    .grid_row(0)
                    .into(),
            ] as [View; _])
    }
}
