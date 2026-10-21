use windows_reactor::*;

pub trait ViewContextExt<C> where C: Component {
    fn with_titlebar_text(&mut self, backdrop: WindowBackdrop, title: impl Into<String>, content: impl Into<View>) -> View;
    fn with_titlebar(&mut self, backdrop: WindowBackdrop, titlebar: impl Into<View>, content: impl Into<View>) -> View;

    fn titlebar(&self, title: impl Into<String>) -> TitleBar {
        TitleBar::new()
            .preferred_height(WindowTitleBarHeight::Tall)
            .title(title)
    }
}

impl<C> ViewContextExt<C> for ViewContext<C> where C: Component {
    fn with_titlebar_text(&mut self, backdrop: WindowBackdrop, title: impl Into<String>, content: impl Into<View>) -> View {
        let title = title.into();

        self.window_visuals(WindowVisuals::new().backdrop(backdrop));
        self.window_title(title.clone());

        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                self.titlebar(title),
                Border::new().grid_row(1).content(content)
            ))
    }

    fn with_titlebar(&mut self, backdrop: WindowBackdrop, titlebar: impl Into<View>, content: impl Into<View>) -> View {
        self.window_visuals(
            WindowVisuals::new()
                // .theme(WindowTheme::Light)
                .backdrop(backdrop)
        );

        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                titlebar,
                Border::new().grid_row(1).content(content),
            ))
    }
}

