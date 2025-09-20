use crate::{
    behavior::{common, conditional::Conditional, write_rules::WriteRules},
    config::{color::Color, icon::Icon, modifier::Modifier, sound::Sound, theme::Theme},
    constants::rules::{ENCHANTED, FRACTURED, INFLUENCED, REPLICA, SYNTHESIZED},
};
use serde_derive::Deserialize;

#[derive(Default, Deserialize, PartialEq)]
pub struct Rule {
    pub name: Option<String>,
    pub items: Option<Vec<String>>,
    pub classes: Option<Vec<String>>,
    pub rarity: Option<String>,
    pub theme: Option<Theme>,
    pub size: Option<u8>,
    pub sound: Option<Sound>,
    pub icon: Option<Icon>,
    pub beam: Option<String>,
    pub is_synthesised: Option<bool>,
    pub is_fractured: Option<bool>,
    pub is_influenced: Option<bool>,
    pub is_enchanted: Option<bool>,
    pub is_veiled: Option<bool>,
    pub is_replica: Option<bool>,
    pub good_mods: Option<u8>,
    pub corrupted_mods: Option<u8>,
    pub quality: Option<u8>,
    pub map_tier: Option<u8>,
    pub links: Option<u8>,
    pub item_level: Option<u8>,
    pub item_tier: Option<u8>,
    pub stack_size: Option<u16>,
    pub strict: Option<bool>,
    pub hide: Option<bool>,
}

impl Rule {
    pub fn get_rule(&self, palette: Vec<Color>, mods: &Option<Vec<Modifier>>) -> String {
        if self.is_default() {
            // if it's a default rule, don't write anything
            String::new()
        } else {
            [
                self.write_rule_name(self.name.clone(), self.hide),
                self.write_list_rule("Class", self.classes.clone(), self.strict),
                self.write_list_rule("BaseType", self.items.clone(), self.strict),
                self.write_optional_rule("Rarity", &self.rarity),
                self.write_optional_rule("MapTier >=", &self.map_tier),
                self.write_optional_rule("ItemLevel >=", &self.item_level),
                self.write_optional_rule("UnidentifiedItemTier >=", &self.item_tier),
                self.write_optional_rule("Quality >=", &self.quality),
                self.write_explicit_mods_rule(
                    &self.is_veiled,
                    &self.good_mods,
                    &self.classes,
                    mods,
                ),
                self.write_rule(FRACTURED, self.is_fractured),
                self.write_rule(INFLUENCED, self.is_influenced),
                self.write_rule(SYNTHESIZED, self.is_synthesised),
                self.write_rule(ENCHANTED, self.is_enchanted),
                self.write_rule(REPLICA, self.is_replica),
                self.write_corrupted_mods_rule(&self.corrupted_mods),
                self.write_optional_rule("LinkedSockets", &self.links),
                self.write_optional_rule("StackSize >=", &self.stack_size),
                self.write_optional_rule("SetFontSize", &self.size),
                self.write_color_rules(palette, &self.theme),
                format!("{}", self.sound.clone().unwrap_or_default()),
                format!("{}", self.icon.clone().unwrap_or_default()),
                self.write_optional_rule(
                    "PlayEffect",
                    &Some(common::capitalize(&self.beam.clone().unwrap_or_default())),
                )
                .only_if(!self.beam.clone().unwrap_or_default().is_default()),
            ]
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| format!("{line}\n"))
            .collect::<String>()
        }
    }
}

impl WriteRules for Rule {}
