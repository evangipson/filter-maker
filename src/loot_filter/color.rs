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

pub const YELLOW: Color = Color::new(255, 255, 0, 255);
pub const RED: Color = Color::new(255, 0, 0, 255);
pub const GREEN: Color = Color::new(0, 255, 0, 255);
pub const PURPLE: Color = Color::new(0, 255, 255, 255);
pub const BLUE: Color = Color::new(0, 0, 255, 255);
pub const WHITE: Color = Color::new(255, 255, 255, 255);
pub const BLACK: Color = Color::new(0, 0, 0, 255);
pub const TRANSPARENT: Color = Color::new(0, 0, 0, 0);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.r, self.g, self.b, self.a)
    }
}
