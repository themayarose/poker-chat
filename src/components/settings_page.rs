use windows_reactor::*;

use crate::helpers::typography::*;

pub struct SettingsPage {
}

#[derive(Clone, PartialEq)]
pub struct Input {
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
            .into()
    }
}
