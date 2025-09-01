use crate::config::{color::Color, modifier::Modifier, style::Style};
use serde_derive::Deserialize;

#[derive(Default, Deserialize, PartialEq)]
pub struct BaseConfig {
    pub destination: String,
    pub palette: Vec<Color>,
    pub styles: Vec<Style>,
    pub mods: Option<Vec<Modifier>>,
}
