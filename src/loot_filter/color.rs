use std::fmt::Display;

#[derive(Default, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub const RARE_YELLOW: Color = Color::new(255, 255, 119, 255);
pub const DIVINE_RED: Color = Color::new(191, 0, 0, 255);
pub const QUEST_GREEN: Color = Color::new(27, 119, 65, 255);
pub const CURRENCY_SALMON: Color = Color::new(254, 128, 128, 255);
pub const DARK_PINK: Color = Color::new(211, 84, 25, 255);
pub const PURPLE: Color = Color::new(142, 68, 173, 255);
pub const MAGIC_BLUE: Color = Color::new(136, 136, 255, 255);
pub const DARK_MAGIC_BLUE: Color = Color::new(41, 128, 185, 255);
pub const UNIQUE_ORANGE: Color = Color::new(175, 96, 37, 255);
pub const KALGUUR_GOLD: Color = Color::new(187, 165, 61, 255);
pub const GEM_TEAL: Color = Color::new(7, 122, 115, 255);
pub const NORMAL_WHITE: Color = Color::new(200, 200, 200, 255);
pub const BLACK: Color = Color::new(42, 42, 42, 255);
pub const FADED_BLACK: Color = Color::new(42, 42, 42, 150);
pub const TRANSPARENT: Color = Color::new(0, 0, 0, 0);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.r, self.g, self.b, self.a)
    }
}
