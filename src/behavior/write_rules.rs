use crate::{
    behavior::{common, conditional::Conditional},
    config::{color::Color, modifier::Modifier, theme::Theme},
    constants::rules::{CORRUPTED, VEILED_MOD},
};
use std::fmt::Display;

/// [`WriteRules`] represents a collection of functions to write out item filter rules.
pub trait WriteRules {
    /// [`WriteRules::write_rule`] will write `rule` as an item filter rule based on `condition`.
    fn write_rule(&self, rule: &str, condition: Option<bool>) -> String {
        rule.to_string().only_if(condition.unwrap_or_default())
    }

    /// [`WriteRules::write_value_rule`] will write `rule` as an item filter rule using `value`.
    fn write_value_rule<T: Default + Display + PartialEq>(&self, rule: &str, value: &T) -> String {
        format!("{rule} {value}").if_not_default(value)
    }

    /// [`WriteRules::write_optional_rule`] will write `rule` as an item filter rule using `value`,
    /// if `value` has a value. Otherwise, will just return an empty [`String`].
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

    /// [`WriteRules::write_list_rule`] will write `rule` as an item filter rule using `list`, with
    /// the option to define the comparator using the `strict_equals` flag. If `list` is [`None`],
    /// returns an empty [`String`].
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

    /// [`WriteRules::write_rarity_rule`] will write a rarity item filter rule using `rarity`, if
    /// `rarity` has a value. Otherwise, it will just return an empty [`String`].
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

    /// [`WriteRules::write_color_rules`] will write `theme` as an item filter rule using `palette`.
    /// If `theme` is [`None`], returns an empty [`String`].
    fn write_color_rules(&self, palette: Vec<Color>, theme: &Option<Theme>) -> String {
        let theme_value = theme.clone().unwrap_or_default();
        [
            self.write_value_rule(
                "SetTextColor",
                &common::get_color(&palette, &theme_value.font.clone().unwrap_or_default()),
            )
            .only_if(!theme_value.font.is_default()),
            self.write_value_rule(
                "SetBackgroundColor",
                &common::get_color(
                    &palette,
                    &theme_value.background.clone().unwrap_or_default(),
                ),
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

    /// [`WriteRules::write_rule_name`] will write `name` as a "Show" or "Hide" item filter rule name.
    /// If `name` is [`None`], returns an empty [`String`].
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

    /// [`WriteRules::write_explicit_mods_rule`] will write `classes` as a modifier-based item filter rule.
    /// If `good_mods` and `is_veiled` are [`None`], returns an empty [`String`].
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

    /// [`WriteRules::write_corrupted_mods_rule`] will write `corrupted_mods` as an item filter rule.
    /// If `corrupted_mods` is [`None`], returns [`CORRUPTED`].
    fn write_corrupted_mods_rule(&self, corrupted_mods: &Option<u8>) -> String {
        if corrupted_mods.unwrap_or_default() == 1 {
            CORRUPTED.to_string()
        } else {
            self.write_optional_rule("CorruptedMods >=", corrupted_mods)
        }
    }
}

/// [`get_valid_mods`] will search `modifiers` for all "good" valid [`Modifiers`](`Modifier`).
/// If `modifiers` is [`None`], returns an empty [`Vec`].
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

/// [`get_mods_for_classes`] will write `modifiers` as an item filter rule for all `classes`.
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
