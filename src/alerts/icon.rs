use std::fmt::Display;

#[derive(PartialEq)]
pub enum IconSize {
    Hidden,
    Small,
    Medium,
    Large,
}

impl Display for IconSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconSize::Hidden => "",
                IconSize::Small => "2",
                IconSize::Medium => "1",
                IconSize::Large => "0",
            }
        )
    }
}

#[derive(PartialEq)]
pub enum IconType {
    None,
    Circle,
    Diamond,
    Hexagon,
    Square,
    Star,
    Triangle,
    Cross,
    Moon,
    Raindrop,
    Kite,
    Pentagon,
    UpsideDownHouse,
}

impl Display for IconType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconType::None => "",
                IconType::Circle => "Circle",
                IconType::Diamond => "Diamond",
                IconType::Hexagon => "Hexagon",
                IconType::Square => "Square",
                IconType::Star => "Star",
                IconType::Triangle => "Triangle",
                IconType::Cross => "Cross",
                IconType::Moon => "Moon",
                IconType::Raindrop => "Raindrop",
                IconType::Kite => "Kite",
                IconType::Pentagon => "Pentagon",
                IconType::UpsideDownHouse => "UpsideDownHouse",
            }
        )
    }
}

#[derive(PartialEq)]
pub enum IconColor {
    None,
    Red,
    Green,
    Blue,
    Brown,
    White,
    Yellow,
    Cyan,
    Grey,
    Orange,
    Pink,
    Purple,
}

impl Display for IconColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconColor::None => "",
                IconColor::Red => "Red",
                IconColor::Green => "Green",
                IconColor::Blue => "Blue",
                IconColor::Brown => "Brown",
                IconColor::White => "White",
                IconColor::Yellow => "Yellow",
                IconColor::Cyan => "Cyan",
                IconColor::Grey => "Grey",
                IconColor::Orange => "Orange",
                IconColor::Pink => "Pink",
                IconColor::Purple => "Purple",
            }
        )
    }
}

#[derive(PartialEq)]
pub struct AlertIcon {
    pub size: IconSize,
    pub color: IconColor,
    pub icon: IconType,
}

impl AlertIcon {
    pub const NONE: AlertIcon = AlertIcon::new(IconSize::Hidden, IconColor::None, IconType::None);
    pub const SMALL_YELLOW_CIRCLE: AlertIcon =
        AlertIcon::new(IconSize::Small, IconColor::Yellow, IconType::Circle);
    pub const BIG_RED_STAR: AlertIcon =
        AlertIcon::new(IconSize::Large, IconColor::Red, IconType::Star);
    pub const YELLOW_STAR: AlertIcon =
        AlertIcon::new(IconSize::Medium, IconColor::Yellow, IconType::Star);
    pub const PURPLE_KITE: AlertIcon =
        AlertIcon::new(IconSize::Medium, IconColor::Purple, IconType::Kite);
    pub const SMALL_RED_CIRCLE: AlertIcon =
        AlertIcon::new(IconSize::Small, IconColor::Red, IconType::Circle);
    pub const SMALL_WHITE_CIRCLE: AlertIcon =
        AlertIcon::new(IconSize::Small, IconColor::White, IconType::Circle);

    const fn new(size: IconSize, color: IconColor, icon: IconType) -> Self {
        Self { size, color, icon }
    }
}

impl Display for AlertIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alert_icon = if self.size == IconSize::Hidden
            || self.color == IconColor::None
            || self.icon == IconType::None
        {
            String::new()
        } else {
            format!("MinimapIcon {} {} {}", self.size, self.color, self.icon)
        };

        write!(f, "{alert_icon}")
    }
}
