use crate::{
    behavior::{conditional::Conditional, write_rules::WriteRules},
    config::{color::Color, modifier::Modifier, theme::Theme},
    constants::rules::{ENCHANTED, FOULBORN, FRACTURED, INFLUENCED, REPLICA, SYNTHESIZED},
};
use serde_derive::Deserialize;

#[derive(Default, Deserialize, PartialEq)]
pub struct Style {
    pub name: Option<String>,
    pub items: Option<Vec<String>>,
    pub classes: Option<Vec<String>>,
    pub rarity: Option<String>,
    pub font: Option<String>,
    pub background: Option<String>,
    pub outline: Option<String>,
    pub size: Option<u8>,
    pub is_synthesised: Option<bool>,
    pub is_fractured: Option<bool>,
    pub is_influenced: Option<bool>,
    pub is_enchanted: Option<bool>,
    pub is_veiled: Option<bool>,
    pub is_replica: Option<bool>,
    pub is_foulborn: Option<bool>,
    pub good_mods: Option<u8>,
    pub corrupted_mods: Option<u8>,
    pub item_level: Option<u8>,
    pub gem_level: Option<u8>,
    pub item_tier: Option<u8>,
    pub strict: Option<bool>,
}

impl Style {
    pub fn get_style(&self, palette: Vec<Color>, mods: &Option<Vec<Modifier>>) -> String {
        if self.is_default() {
            // if it's a default rule, don't write anything
            String::new()
        } else {
            let theme = Theme::new(&self.font, &self.background, &self.outline);
            [
                self.write_rule_name(self.name.clone(), self.strict),
                self.write_list_rule("Class", self.classes.clone(), self.strict),
                self.write_list_rule("BaseType", self.items.clone(), self.strict),
                self.write_explicit_mods_rule(
                    &self.is_veiled,
                    &self.good_mods,
                    &self.classes,
                    mods,
                ),
                self.write_optional_rule("ItemLevel >=", &self.item_level),
                self.write_optional_rule("GemLevel >=", &self.gem_level),
                self.write_optional_rule("UnidentifiedItemTier >=", &self.item_tier),
                self.write_rarity_rule(self.rarity.clone()),
                self.write_rule(FRACTURED, self.is_fractured),
                self.write_rule(INFLUENCED, self.is_influenced),
                self.write_rule(SYNTHESIZED, self.is_synthesised),
                self.write_rule(ENCHANTED, self.is_enchanted),
                self.write_rule(REPLICA, self.is_replica),
                self.write_rule(FOULBORN, self.is_foulborn),
                self.write_corrupted_mods_rule(&self.corrupted_mods),
                self.write_optional_rule("SetFontSize", &self.size),
                self.write_color_rules(palette, &theme),
                "Continue".to_string(),
            ]
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| format!("{line}\n"))
            .collect::<String>()
        }
    }
}

impl WriteRules for Style {}
