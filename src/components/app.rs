use std::collections::HashMap;
use std::sync::OnceLock;

use windows_reactor::*;

use crate::components::main_window::MainWindow;
use crate::helpers::styles;
use crate::helpers::theme_resources::theme_resource;

pub struct App {
}

#[derive(Debug, Clone)]
pub enum Message {
}

static RESOURCES: OnceLock<HashMap<String, styles::ResourceValue>> = OnceLock::new();

impl App {
    pub fn get_resources() -> &'static HashMap<String, styles::ResourceValue> {
        RESOURCES.get_or_init(|| {
            let styles: [&str; _] = [
                "TabViewItemHeaderBackgroundSelected",
                "AccentFillColorTertiaryBrush",
                "TextOnAccentFillColorPrimaryBrush",
                "CardBackgroundFillColorSecondaryBrush",
                "SolidBackgroundFillColorBaseAltBrush",
                "LayerOnMicaBaseAltFillColorDefaultBrush",
                "CardStrokeColorDefaultBrush",
            ];

            styles.iter()
                .filter_map(|key| {
                    theme_resource(key, WindowTheme::System)
                        .ok()
                        .flatten()
                        .and_then(|resource| Some((key.to_string(), resource.into())))
                })
                .collect()
        })
    }

    pub fn get_resource(key: &str) -> Option<&styles::ResourceValue> {
        Self::get_resources().get(&key.to_string())
    }

    pub fn get_brush(key: &str) -> Option<Brush> {
        Self::get_resource(key)
            .and_then(|res| match res {
                styles::ResourceValue::Color(color) => Some(color.to_owned().into()),
                _ => None,
            })
    }

    pub fn get_thickness(key: &str) -> Option<Thickness> {
        Self::get_resource(key)
            .and_then(|res| match res {
                styles::ResourceValue::Thickness(thickness) => Some(thickness.to_owned().into()),
                _ => None,
            })
    }

    pub fn get_radius(key: &str) -> Option<CornerRadius> {
        Self::get_resource(key)
            .and_then(|res| match res {
                styles::ResourceValue::CornerRadius(radius) => Some(radius.to_owned().into()),
                _ => None,
            })
    }
}

impl Component for App {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        App { }
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {

    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::component::<MainWindow>(())
    }
}
