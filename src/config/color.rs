use crate::color::custom_color::CustomColor;
use serde_derive::Deserialize;

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Color {
    pub name: Option<String>,
    pub color: Option<CustomColor>,
}
