use serde_derive::Deserialize;
use std::fmt::Display;

/// [`CustomColor`] represents a dynamic color with a red, green, blue, and alpha value.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct CustomColor {
    /// [`CustomColor::r`] is the red value of a color, from 0-255.
    pub r: u8,
    /// [`CustomColor::g`] is the green value of a color, from 0-255.
    pub g: u8,
    /// [`CustomColor::b`] is the blue value of a color, from 0-255.
    pub b: u8,
    /// [`CustomColor::a`] is the transparency (alpha) value of a color, from 0-255.
    pub a: u8,
}

impl CustomColor {
    /// [`CustomColor::TRANSPARENT`] represents complete transparency (or no color).
    pub const TRANSPARENT: CustomColor = CustomColor::new(0, 0, 0, 0);

    /// [`CustomColor::new`] will create a new [`CustomColor`].
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Implement [`Display`] for [`CustomColor`].
impl Display for CustomColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.r, self.g, self.b, self.a)
    }
}
