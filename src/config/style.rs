use crate::{
    behavior::conditional::Conditional,
    config::{color::Color, common},
    constants::rules::{ENCHANTED, FRACTURED, INFLUENCED, REPLICA, SYNTHESIZED},
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
    pub has_tier_1_mods: Option<u8>,
    pub corrupted_mods: Option<u8>,
    pub item_level: Option<u8>,
    pub strict: Option<bool>,
}

impl Style {
    pub fn get_style(&self, palette: Vec<Color>) -> String {
        if self.is_default() {
            // if it's a default rule, don't write anything
            String::new()
        } else {
            [
                common::get_name_display(self.name.clone(), self.strict),
                common::get_class_display(self.classes.clone(), self.strict),
                common::get_item_display(self.items.clone(), self.strict),
                common::get_explicit_mods(&self.is_veiled, &self.has_tier_1_mods),
                format!("ItemLevel >= {}", self.item_level.unwrap_or_default())
                    .only_if(!self.item_level.unwrap_or_default().is_default()),
                format!(
                    "Rarity >= {}",
                    common::capitalize(&self.rarity.clone().unwrap_or("Normal".to_string()))
                )
                .only_if(
                    &self
                        .rarity
                        .clone()
                        .unwrap_or("Normal".to_string())
                        .to_lowercase()
                        != "none",
                ),
                FRACTURED
                    .to_string()
                    .only_if(self.is_fractured.unwrap_or_default()),
                INFLUENCED
                    .to_string()
                    .only_if(self.is_influenced.unwrap_or_default()),
                SYNTHESIZED
                    .to_string()
                    .only_if(self.is_synthesised.unwrap_or_default()),
                ENCHANTED
                    .to_string()
                    .only_if(self.is_enchanted.unwrap_or_default()),
                REPLICA
                    .to_string()
                    .only_if(self.is_replica.unwrap_or_default()),
                common::get_display("CorruptedMods >=", &self.corrupted_mods),
                common::get_display("SetFontSize", &self.size),
                format!(
                    "SetTextColor {}",
                    common::get_color(&palette, &self.font.clone().unwrap_or_default())
                )
                .only_if(!&self.font.clone().unwrap_or_default().is_default()),
                format!(
                    "SetBackgroundColor {}",
                    common::get_color(&palette, &self.background.clone().unwrap_or_default())
                )
                .only_if(!&self.background.clone().unwrap_or_default().is_default()),
                format!(
                    "SetBorderColor {}",
                    common::get_color(&palette, &self.outline.clone().unwrap_or_default())
                )
                .only_if(!&self.outline.clone().unwrap_or_default().is_default()),
                "Continue".to_string(),
            ]
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| line + "\n")
            .collect::<String>()
            .to_string()
        }
    }
}
