use windows_bindgen::*;
use windows_reactor_setup;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    builder()
        .filters([
            "Microsoft.UI.Xaml.Application",
            "Microsoft.UI.Xaml.Application::{Resources, RequestedTheme}",
            "Microsoft.UI.Xaml.IApplicationFactory",
            "Microsoft.UI.Xaml.IApplicationStatics",
            "Microsoft.UI.Xaml.IDependencyObjectFactory",
            "Microsoft.UI.Xaml.IDependencyPropertyStatics",
            "Microsoft.UI.Xaml.ApplicationTheme",
            "Microsoft.UI.Xaml.ResourceDictionary",
            "Microsoft.UI.Xaml.ResourceDictionary::{HasKey, ThemeDictionaries, Lookup, MergedDictionaries}",
            "Microsoft.UI.Xaml.IPropertyMetadataStatics",
            "Microsoft.UI.Xaml.IPropertyMetadataFactory",
            "Microsoft.UI.Xaml.IResourceDictionaryFactory",
            "Microsoft.UI.Xaml.Media.SolidColorBrush",
            "Microsoft.UI.Xaml.Media.SolidColorBrush::Color",
            "Microsoft.UI.Xaml.Media.SolidColorBrush::Opacity",
            "Microsoft.UI.Xaml.Media.IBrushFactory",
            "Microsoft.UI.Xaml.Media.IBrushStatics",
            "Microsoft.UI.Xaml.Media.ISolidColorBrushFactory",
            "Microsoft.UI.Xaml.Media.ISolidColorBrushStatics",
            "Microsoft.UI.Xaml.Thickness",
            "Microsoft.UI.Xaml.CornerRadius",
            "Windows.Foundation.PropertyValue",
            "Windows.Foundation.IPropertyValueStatics",
            "Windows.Foundation.IUriEscapeStatics",
            "Windows.Foundation.IUriRuntimeClassFactory",
            "Windows.Foundation.IReference",
            "Windows.Foundation.Collections.IKeyValuePair",
            "Windows.Foundation.Collections.IIterable",
            "Windows.Foundation.Collections.IMap",
            "Windows.Foundation.Collections.IMap::{HasKey, Lookup}",
        ])
        .input_default()
        .input("winmd/Microsoft.Foundation.winmd")
        .input("winmd/Microsoft.Graphics.winmd")
        .input("winmd/Microsoft.Security.Authentication.OAuth.winmd")
        .input("winmd/Microsoft.UI.winmd")
        .input("winmd/Microsoft.UI.Xaml.winmd")
        .input("winmd/Microsoft.Windows.ApplicationModel.Background.UniversalBGTask.winmd")
        .input("winmd/Microsoft.Windows.ApplicationModel.Background.winmd")
        .input("winmd/Microsoft.Windows.ApplicationModel.DynamicDependency.winmd")
        .input("winmd/Microsoft.Windows.ApplicationModel.Resources.winmd")
        .input("winmd/Microsoft.Windows.ApplicationModel.WindowsAppRuntime.winmd")
        .input("winmd/Microsoft.Windows.AppLifecycle.winmd")
        .input("winmd/Microsoft.Windows.AppNotifications.Builder.winmd")
        .input("winmd/Microsoft.Windows.AppNotifications.winmd")
        .input("winmd/Microsoft.Windows.BadgeNotifications.winmd")
        .input("winmd/Microsoft.Windows.Foundation.winmd")
        .input("winmd/Microsoft.Windows.Globalization.winmd")
        .input("winmd/Microsoft.Windows.Management.Deployment.winmd")
        .input("winmd/Microsoft.Windows.Media.Capture.winmd")
        .input("winmd/Microsoft.Windows.PushNotifications.winmd")
        .input("winmd/Microsoft.Windows.Security.AccessControl.winmd")
        .input("winmd/Microsoft.Windows.Storage.Pickers.winmd")
        .input("winmd/Microsoft.Windows.Storage.winmd")
        .input("winmd/Microsoft.Windows.System.Power.winmd")
        .input("winmd/Microsoft.Windows.System.winmd")
        .output("src/helpers/bindings.rs")
        .full()
        .write();

    windows_reactor_setup::as_self_contained();
}
