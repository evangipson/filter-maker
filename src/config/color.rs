use crate::color::custom_color::CustomColor;
use serde_derive::Deserialize;

/// [`Color`] is a rust representation of a color defined in the TOML filter
/// configuration file.
#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Color {
    /// [`Color::name`] is the optional name of a color.
    pub name: Option<String>,
    /// [`Color::color`] is the optional [`CustomColor`] value of a color.
    pub color: Option<CustomColor>,
}
