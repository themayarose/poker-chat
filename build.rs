use windows_bindgen::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    builder()
        .filters([
            "Application",
            "Application::{Resources, RequestedTheme}",
            "IApplicationFactory",
            "IApplicationStatics",
            "IDependencyObjectFactory",
            "IDependencyPropertyStatics",
            "ApplicationTheme",
            "ResourceDictionary",
            "ResourceDictionary::{HasKey, ThemeDictionaries, Lookup, MergedDictionaries}",
            "IPropertyMetadataStatics",
            "IPropertyMetadataFactory",
            "IResourceDictionaryFactory",
            "SolidColorBrush",
            "SolidColorBrush::Color",
            "SolidColorBrush::Opacity",
            "IBrushFactory",
            "IBrushStatics",
            "ISolidColorBrushFactory",
            "ISolidColorBrushStatics",
            "Thickness",
            "CornerRadius",
            "IKeyValuePair",
            "Windows.Foundation.PropertyValue",
            "Windows.Foundation.IPropertyValueStatics",
            "Windows.Foundation.IUriEscapeStatics",
            "Windows.Foundation.IUriRuntimeClassFactory",
            "Windows.Foundation.IReference",
            "Windows.Foundation.Collections.IIterable",
            "Windows.Foundation.Collections.IMap",
            "Windows.Foundation.Collections.IMap::{HasKey, Lookup}",
        ])
        .full()
        .output("src/helpers/bindings.rs")
        .write();
}
