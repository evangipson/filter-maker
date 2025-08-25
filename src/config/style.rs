use crate::{
    behavior::conditional::Conditional,
    config::{color::Color, common},
    constants::rules::{FRACTURED, INFLUENCED, SYNTHESIZED},
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
    pub is_synthesised: Option<bool>,
    pub is_fractured: Option<bool>,
    pub is_influenced: Option<bool>,
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
                //common::get_display("Rarity", &self.rarity),
                FRACTURED
                    .to_string()
                    .only_if(self.is_fractured.unwrap_or_default()),
                INFLUENCED
                    .to_string()
                    .only_if(self.is_influenced.unwrap_or_default()),
                SYNTHESIZED
                    .to_string()
                    .only_if(self.is_synthesised.unwrap_or_default()),
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
