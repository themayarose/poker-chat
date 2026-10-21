//! One-off generator for `src/winui.rs`.
//!
//! Put this in its own small crate (for example `tools/gen-bindings/src/main.rs`) that depends on
//! `windows-bindgen`, then run it from your project root:
//!
//!     cargo run -p gen-bindings -- path/to/winmd-folder
//!
//! `winmd-folder` must contain the `.winmd` files from the Windows App SDK NuGet packages
//! (`Microsoft.UI.Xaml.winmd`, `Microsoft.UI.winmd`, `Microsoft.Windows.ApplicationModel.Resources.winmd`
//! and the other files that ship beside them). Copy them into one folder first.
//!
//! The generated file refers to `windows-core`, and possibly `windows-collections`,
//! `windows-future` and `windows-numerics`. Add whichever ones the compiler asks for.

fn main() {
    let winmd_dir = std::env::args()
        .nth(1)
        .expect("usage: gen-bindings <folder containing Windows App SDK .winmd files>");

    windows_bindgen::bindgen([
        // Windows.* metadata bundled with windows-bindgen, plus the WinUI metadata.
        "--in",
        "default",
        "--in",
        winmd_dir.as_str(),
        "--out",
        "src/winui.rs",
        "--filter",
        "Microsoft.UI.Xaml.Application",
        "Microsoft.UI.Xaml.ApplicationTheme",
        "Microsoft.UI.Xaml.ResourceDictionary",
        "Microsoft.UI.Xaml.Media.SolidColorBrush",
        "Microsoft.UI.Xaml.Thickness",
        "Microsoft.UI.Xaml.CornerRadius",
        "Windows.Foundation.PropertyValue",
        "Windows.Foundation.IReference",
    ])
    .unwrap();
}
