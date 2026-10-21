//! Look up WinUI theme resources by key for a given window theme and return them as
//! `windows_reactor::ResourceValue`.
//!
//! `windows-reactor` keeps its WinUI bindings private, and the `windows` crate does not contain
//! `Microsoft.UI.Xaml`, so this module expects its own bindings in `winui.rs`, generated with
//! `gen_bindings.rs`.
//!
//! Call from the UI thread, after the WinUI `Application` has been created (for example from a
//! component's `view`/`update`, or any callback that runs on the UI thread).

use crate::helpers::winui::Microsoft::UI::Xaml::{
    Application, ApplicationTheme, CornerRadius as XamlCornerRadius, Media::SolidColorBrush,
    ResourceDictionary, Thickness as XamlThickness,
};
use crate::helpers::winui::Windows::Foundation::{IReference, PropertyValue};
use crate::helpers::winui::Windows::UI::Color as WinColor;
use windows_core::{HSTRING, IInspectable, Interface, Result};
use windows_reactor::{Color, CornerRadius, ResourceValue, Thickness, WindowTheme};

/// Resolves `key` (for example `"CardBackgroundFillColorDefaultBrush"`) in the given theme.
///
/// * `Ok(Some(_))`: the resource exists and has a type `ResourceValue` can represent
///   (`SolidColorBrush`/`Color` -> `Color`, `Thickness`, `CornerRadius`).
/// * `Ok(None)`: no such key, or it holds something else (gradient/acrylic brush, double, ...).
/// * `Err(_)`: a WinRT call failed (wrong thread, no `Application`, ...).
pub fn theme_resource(key: &str, theme: WindowTheme) -> Result<Option<ResourceValue>> {
    let app = Application::Current()?;

    let theme_name = match theme {
        WindowTheme::Light => "Light",
        WindowTheme::Dark => "Dark",
        // `System` follows whatever theme the application itself is running with.
        WindowTheme::System => {
            if app.RequestedTheme()? == ApplicationTheme::Dark {
                "Dark"
            } else {
                "Light"
            }
        }
    };

    // WinUI looks in ThemeDictionaries[<theme>] first and then ThemeDictionaries["Default"].
    let themes = [boxed(theme_name)?, boxed("Default")?];
    let key = boxed(key)?;

    Ok(find(&app.Resources()?, &themes, &key)?.and_then(|value| convert(&value)))
}

fn boxed(value: &str) -> Result<IInspectable> {
    PropertyValue::CreateString(&HSTRING::from(value))
}

/// Mirrors the XAML lookup order for what we need: theme dictionaries, own entries, then merged
/// dictionaries from last to first (the last merged dictionary wins).
fn find(
    dictionary: &ResourceDictionary,
    themes: &[IInspectable; 2],
    key: &IInspectable,
) -> Result<Option<IInspectable>> {
    let theme_dictionaries = dictionary.ThemeDictionaries()?;
    for name in themes {
        if theme_dictionaries.HasKey(name)? {
            if let Ok(inner) = theme_dictionaries.Lookup(name)?.cast::<ResourceDictionary>() {
                if let Some(found) = find(&inner, themes, key)? {
                    return Ok(Some(found));
                }
            }
        }
    }

    if dictionary.HasKey(key)? {
        return dictionary.Lookup(key).map(Some);
    }

    let merged = dictionary.MergedDictionaries()?;
    for index in (0..merged.Size()?).rev() {
        if let Some(found) = find(&merged.GetAt(index)?, themes, key)? {
            return Ok(Some(found));
        }
    }

    Ok(None)
}

fn convert(value: &IInspectable) -> Option<ResourceValue> {
    if let Ok(brush) = value.cast::<SolidColorBrush>() {
        let color = brush.Color().ok()?;
        let opacity = brush.Opacity().unwrap_or(1.0);
        return Some(argb(color, opacity).into());
    }
    if let Ok(color) = value.cast::<IReference<WinColor>>().and_then(|v| v.Value()) {
        return Some(argb(color, 1.0).into());
    }
    if let Ok(t) = value.cast::<IReference<XamlThickness>>().and_then(|v| v.Value()) {
        return Some(Thickness::new(t.Left, t.Top, t.Right, t.Bottom).into());
    }
    if let Ok(r) = value.cast::<IReference<XamlCornerRadius>>().and_then(|v| v.Value()) {
        return Some(CornerRadius::new(r.TopLeft, r.TopRight, r.BottomRight, r.BottomLeft).into());
    }
    None
}

/// Folds `Brush.Opacity` into alpha so the result matches what the brush actually paints.
fn argb(color: WinColor, opacity: f64) -> Color {
    let alpha = (f64::from(color.A) * opacity.clamp(0.0, 1.0)).round() as u8;
    Color::argb(alpha, color.R, color.G, color.B)
}

#[allow(dead_code)]
fn example() -> Result<()> {
    let card = theme_resource("CardBackgroundFillColorDefaultBrush", WindowTheme::Dark)?;
    // Expected shape: Some(ResourceValue::Color(Color::argb(a, r, g, b)))
    println!("{card:?}");
    Ok(())
}
