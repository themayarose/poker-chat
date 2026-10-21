use windows_reactor::*;

pub fn display(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(68.0)
        .font_weight(FontWeight::SEMI_BOLD)
}

pub fn title_large(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(40.0)
        .font_weight(FontWeight::SEMI_BOLD)
}

pub fn title(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(28.0)
        .font_weight(FontWeight::SEMI_BOLD)
}

pub fn subtitle(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(20.0)
        .font_weight(FontWeight::SEMI_BOLD)
}

pub fn body_large_strong(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(18.0)
        .font_weight(FontWeight::SEMI_BOLD)
}

pub fn body_large(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(18.0)
        .font_weight(FontWeight::NORMAL)
}

pub fn body_strong(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(14.0)
        .font_weight(FontWeight::SEMI_BOLD)
}

pub fn body(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(14.0)
        .font_weight(FontWeight::NORMAL)
}

pub fn caption(text: impl Into<String>) -> TextBlock {
    TextBlock::new()
        .text(text.into())
        .font_size(12.0)
        .font_weight(FontWeight::NORMAL)
}

