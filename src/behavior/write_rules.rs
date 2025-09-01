use crate::{
    behavior::{common, conditional::Conditional},
    config::{color::Color, modifier::Modifier, theme::Theme},
    constants::rules::{CORRUPTED, VEILED_MOD},
};
use std::fmt::Display;

pub trait WriteRules {
    fn write_rule(&self, rule: &str, condition: Option<bool>) -> String {
        rule.to_string().only_if(condition.unwrap_or_default())
    }

    fn write_value_rule<T: Default + Display + PartialEq>(&self, rule: &str, value: &T) -> String {
        format!("{rule} {value}").if_not_default(value)
    }

    fn write_optional_rule<T: Default + Display + PartialEq>(
        &self,
        rule: &str,
        value: &Option<T>,
    ) -> String {
        if let Some(property_value) = value {
            format!("{rule} {property_value}").if_not_default(value)
        } else {
            String::new()
        }
    }

    fn write_list_rule(
        &self,
        rule: &str,
        list: Option<Vec<String>>,
        strict_equals: Option<bool>,
    ) -> String {
        if let Some(list_values) = list {
            format!(
                "{rule} {}{}",
                if strict_equals.unwrap_or(true) {
                    "== "
                } else {
                    ""
                },
                list_values
                    .iter()
                    .map(|value| "\"".to_owned() + value + "\" ")
                    .collect::<String>()
                    .trim_end()
            )
            .only_if(!list_values.is_empty())
        } else {
            String::new()
        }
    }

    fn write_rarity_rule(&self, rarity: Option<String>) -> String {
        format!(
            "Rarity >= {}",
            common::capitalize(&rarity.clone().unwrap_or("Normal".to_string()))
        )
        .only_if(
            &rarity
                .clone()
                .unwrap_or("Normal".to_string())
                .to_lowercase()
                != "none",
        )
    }

    fn write_color_rules(&self, palette: Vec<Color>, theme: &Option<Theme>) -> String {
        let theme_value = theme.clone().unwrap_or_default();
        [
            self.write_value_rule(
                "SetTextColor",
                &common::get_color(&palette, &theme_value.font.clone()),
            )
            .only_if(!theme_value.font.is_default()),
            self.write_value_rule(
                "SetBackgroundColor",
                &common::get_color(&palette, &theme_value.background.clone()),
            )
            .only_if(!theme_value.background.is_default()),
            self.write_value_rule(
                "SetBorderColor",
                &common::get_color(&palette, &theme_value.outline.clone().unwrap_or_default()),
            )
            .only_if(!theme_value.outline.unwrap_or_default().is_default()),
        ]
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| format!("{x}\n"))
        .collect::<String>()
        .strip_suffix("\n")
        .unwrap_or_default()
        .to_string()
    }

    fn write_rule_name(&self, name: Option<String>, hide: Option<bool>) -> String {
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

    fn write_explicit_mods_rule(
        &self,
        is_veiled: &Option<bool>,
        good_mods: &Option<u8>,
        classes: &Option<Vec<String>>,
        mods: &Option<Vec<Modifier>>,
    ) -> String {
        if is_veiled.is_default() && good_mods.is_default() {
            String::new()
        } else if good_mods.is_default() {
            format!("HasExplicitMod {VEILED_MOD}")
        } else {
            format!(
                "HasExplicitMod >={} {}",
                good_mods.unwrap_or(1),
                if !is_veiled.is_default() && !good_mods.is_default() {
                    get_mods_for_classes(mods, classes) + " " + VEILED_MOD
                } else {
                    get_mods_for_classes(mods, classes)
                }
            )
        }
    }

    fn write_corrupted_mods_rule(&self, corrupted_mods: &Option<u8>) -> String {
        if corrupted_mods.unwrap_or_default() == 1 {
            CORRUPTED.to_string()
        } else {
            self.write_optional_rule("CorruptedMods >=", corrupted_mods)
        }
    }
}

fn get_valid_mods(modifiers: &Option<Vec<Modifier>>) -> Vec<&Modifier> {
    if let Some(mods_value) = modifiers {
        mods_value
            .iter()
            .filter(|modifier| modifier.good_mods.is_some())
            .collect::<Vec<&Modifier>>()
    } else {
        vec![]
    }
}

fn get_mods_for_classes(
    modifiers: &Option<Vec<Modifier>>,
    classes: &Option<Vec<String>>,
) -> String {
    get_valid_mods(modifiers)
        .iter()
        .filter(|modifier| {
            if let Some(classes_value) = classes {
                if let Some(modifier_classes) = &modifier.classes {
                    modifier_classes.iter().any(|c| classes_value.contains(c))
                } else {
                    true
                }
            } else {
                true
            }
        })
        .flat_map(|modifier| {
            modifier
                .good_mods
                .clone()
                .unwrap()
                .into_iter()
                .map(|good_mod| format!("\"{good_mod}\" "))
        })
        .collect::<String>()
        .trim_end()
        .to_string()
}
