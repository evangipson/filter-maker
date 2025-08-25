use crate::config::{color::Color, style::Style};
use serde_derive::Deserialize;

#[derive(Default, Deserialize, PartialEq)]
pub struct BaseConfig {
    pub destination: String,
    pub palette: Vec<Color>,
    pub styles: Vec<Style>,
}
