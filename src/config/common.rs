use crate::{
    behavior::conditional::Conditional, color::custom_color::CustomColor, config::color::Color,
};
use std::fmt::Display;

pub fn get_display<T: Default + Display + Conditional + PartialEq>(
    prefix: &str,
    property: &Option<T>,
) -> String {
    if let Some(property_value) = property {
        format!("{prefix} {property_value}").if_not_default(property)
    } else {
        String::new()
    }
}

pub fn get_name_display(name: Option<String>, hide: Option<bool>) -> String {
    format!(
        "{} # {}",
        match hide {
            Some(value) => match value {
                true => "Hide",
                false => "Show",
            },
            None => "Show",
        },
        match name {
            Some(_) => name.clone().unwrap(),
            None => String::new(),
        }
    )
    .only_if(!name.unwrap_or_default().is_empty())
}

pub fn get_item_display(items: Option<Vec<String>>, strict: Option<bool>) -> String {
    if let Some(item_list) = items {
        format!(
            "BaseType {}{}",
            if strict.unwrap_or(true) { "== " } else { "" },
            item_list
                .iter()
                .map(|item| "\"".to_owned() + item + "\" ")
                .collect::<String>()
                .trim_end()
        )
        .only_if(!item_list.is_empty())
    } else {
        String::new()
    }
}

pub fn get_class_display(classes: Option<Vec<String>>, strict: Option<bool>) -> String {
    if let Some(class_list) = classes {
        format!(
            "Class {}{}",
            if strict.unwrap_or(true) { "== " } else { "" },
            class_list
                .iter()
                .map(|class| "\"".to_owned() + class + "\" ")
                .collect::<String>()
                .trim_end()
        )
        .only_if(!class_list.is_empty())
    } else {
        String::new()
    }
}

pub fn get_color(palette: &[Color], name: &String) -> CustomColor {
    if !name.is_empty() {
        palette
            .iter()
            .find(|p| &p.name.clone().unwrap_or_default() == name)
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
