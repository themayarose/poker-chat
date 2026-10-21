use windows_reactor as win;

#[derive(Clone, PartialEq)]
pub struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Color { alpha, red, green, blue, }
    }
}

impl From<win::Color> for Color {
    fn from(value: win::Color) -> Self {
        Color::new(value.a, value.r, value.g, value.b)
    }
}

impl Into<win::Color> for Color {
    fn into(self) -> win::Color {
        win::Color::argb(self.alpha, self.red, self.green, self.blue)
    }
}

impl Into<win::Brush> for Color {
    fn into(self) -> win::Brush {
        win::Brush::Solid(self.into())
    }
}

#[derive(Clone, PartialEq)]
pub struct Thickness {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Thickness {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Self {
        Thickness { left, top, right, bottom, }
    }
}

impl From<win::Thickness> for Thickness {
    fn from(value: win::Thickness) -> Self {
        Thickness::new(value.left(), value.top(), value.right(), value.bottom())
    }
}

impl Into<win::Thickness> for Thickness {
    fn into(self) -> win::Thickness {
        win::Thickness::new(self.left, self.top, self.right, self.bottom)
    }
}

#[derive(Clone, PartialEq)]
pub struct CornerRadius {
    top_left: f64,
    top_right: f64,
    bottom_right: f64,
    bottom_left: f64,
}

impl CornerRadius {
    pub fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        CornerRadius { top_left, top_right, bottom_right, bottom_left }
    }
}

impl From<win::CornerRadius> for CornerRadius {
    fn from(value: win::CornerRadius) -> Self {
        CornerRadius::new(value.top_left(), value.top_right(), value.bottom_right(), value.bottom_left())
    }
}

impl Into<win::CornerRadius> for CornerRadius {
    fn into(self) -> win::CornerRadius {
        win::CornerRadius::new(self.top_left, self.top_right, self.bottom_right, self.bottom_left)
    }
}


#[derive(Clone, PartialEq)]
pub enum ResourceValue {
    Color(Color),
    Thickness(Thickness),
    CornerRadius(CornerRadius)
}

impl From<win::ResourceValue> for ResourceValue {
    fn from(value: win::ResourceValue) -> Self {
        match value {
            win::ResourceValue::Color(color) => ResourceValue::Color(color.into()),
            win::ResourceValue::Thickness(thickness) => ResourceValue::Thickness(thickness.into()),
            win::ResourceValue::CornerRadius(radius) => ResourceValue::CornerRadius(radius.into()),
        }
    }
}

impl Into<win::ResourceValue> for ResourceValue {
    fn into(self) -> win::ResourceValue {
        match self {
            ResourceValue::Color(color) => win::ResourceValue::Color(color.into()),
            ResourceValue::Thickness(thickness) => win::ResourceValue::Thickness(thickness.into()),
            ResourceValue::CornerRadius(radius) => win::ResourceValue::CornerRadius(radius.into()),
        }
    }
}
