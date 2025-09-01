use crate::{color::custom_color::CustomColor, config::color::Color};

pub fn get_color(palette: &[Color], name: &str) -> CustomColor {
    if !name.is_empty() {
        palette
            .iter()
            .find(|p| p.name.clone().unwrap_or_default().to_lowercase() == name.to_lowercase())
            .cloned()
            .unwrap_or_default()
            .color
            .unwrap_or(CustomColor::TRANSPARENT)
    } else {
        CustomColor::TRANSPARENT
    }
}

pub fn capitalize(s: &str) -> String {
    if s.len() > 1 {
        s[0..1].to_uppercase() + &s[1..]
    } else {
        s.to_uppercase()
    }
}
