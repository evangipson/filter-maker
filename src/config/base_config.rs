use crate::config::{color::Color, modifier::Modifier, style::Style};
use serde_derive::Deserialize;

/// [`BaseConfig`] is a rust representation of the `[configs]` section of the
/// TOML filter configuration file.
#[derive(Default, Deserialize, PartialEq)]
pub struct BaseConfig {
    /// [`BaseConfig::palette`] is a collection of [`Colors`](Color).
    pub palette: Vec<Color>,
    /// [`BaseConfig::styles`] is a collection of [`Styles`](Style).
    pub styles: Vec<Style>,
    /// [`BaseConfig::mods`] is an optional collection of item
    /// [`Modifiers`](Modifier).
    pub mods: Option<Vec<Modifier>>,
}
