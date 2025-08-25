use crate::{
    behavior::conditional::Conditional,
    config::{color::Color, common, icon::Icon, sound::Sound, theme::Theme},
    constants::rules::{FRACTURED, INFLUENCED, SYNTHESIZED},
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
    pub quality: Option<u8>,
    pub map_tier: Option<u8>,
    pub links: Option<u8>,
    pub stack_size: Option<u16>,
    pub strict: Option<bool>,
    pub hide: Option<bool>,
}

impl Rule {
    pub fn get_rule(&self, palette: Vec<Color>) -> String {
        if self.is_default() {
            // if it's a default rule, don't write anything
            String::new()
        } else {
            let theme = self.theme.clone().unwrap_or_default();
            [
                common::get_name_display(self.name.clone(), self.hide),
                common::get_class_display(self.classes.clone(), self.strict),
                common::get_item_display(self.items.clone(), self.strict),
                common::get_display("Rarity", &self.rarity),
                common::get_display("MapTier >=", &self.map_tier),
                common::get_display("Quality >=", &self.quality).if_not_default(&self.quality),
                FRACTURED
                    .to_string()
                    .only_if(self.is_fractured.unwrap_or_default()),
                INFLUENCED
                    .to_string()
                    .only_if(self.is_influenced.unwrap_or_default()),
                SYNTHESIZED
                    .to_string()
                    .only_if(self.is_synthesised.unwrap_or_default()),
                common::get_display("LinkedSockets", &self.links),
                common::get_display("StackSize >=", &self.stack_size),
                common::get_display("SetFontSize", &self.size),
                format!("SetTextColor {}", common::get_color(&palette, &theme.font))
                    .only_if(!theme.font.is_default()),
                format!(
                    "SetBackgroundColor {}",
                    common::get_color(&palette, &theme.background)
                )
                .only_if(!theme.background.is_default()),
                format!(
                    "SetBorderColor {}",
                    common::get_color(&palette, &theme.outline.clone().unwrap_or_default())
                )
                .only_if(!theme.outline.unwrap_or_default().is_default()),
                format!("{}", self.sound.clone().unwrap_or_default()),
                format!("{}", self.icon.clone().unwrap_or_default()),
                format!(
                    "PlayEffect {}",
                    common::capitalize(&self.beam.clone().unwrap_or_default())
                )
                .only_if(!self.beam.clone().unwrap_or_default().is_default()),
            ]
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| line + "\n")
            .collect::<String>()
            .to_string()
        }
    }
}
