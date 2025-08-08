use crate::loot_filter::{
    class::Class,
    color::{self, Color},
    effect::Effect,
    item::Item,
    rarity::Rarity,
};
use std::fmt::Display;

#[derive(Default, PartialEq)]
pub struct Rule {
    pub hide: bool,
    pub name: &'static str,
    pub classes: &'static [Class],
    pub items: Box<[Item]>,
    pub rarity: Rarity,
    pub map_tier: u8,
    pub area_level: u8,
    pub fractured: bool,
    pub influenced: bool,
    pub synthesized: bool,
    pub links: u8,
    pub text_color: Color,
    pub bg_color: Color,
    pub outline_color: Color,
    pub font_size: u8,
    pub effect: Effect,
    pub finalize: bool,
}

impl Rule {
    pub fn ding(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self::new(name, classes, items, rarity)
            .set_color(color::BLACK, color::YELLOW, color::TRANSPARENT)
            .set_effect(Effect::NORMAL_DROP)
    }

    pub fn schwing(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self::new(name, classes, items, rarity)
            .set_color(color::RED, color::WHITE, color::RED)
            .set_effect(Effect::BIG_DROP)
    }

    pub fn ping(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self::new(name, classes, items, rarity)
    }

    pub fn maps(name: &'static str, map_tier: u8, rarity: Rarity) -> Self {
        Self::new(name, &[], Box::new([]), rarity).set_map_tier(map_tier)
    }

    pub fn influenced() -> Self {
        Self::new("Influenced Bases", &[], Box::new([]), Rarity::All).set_influenced(true)
    }

    pub fn synthesized() -> Self {
        Self::new("Synthesized Bases", &[], Box::new([]), Rarity::All).set_synthesized(true)
    }

    pub fn fractured() -> Self {
        Self::new("Fractured Bases", &[], Box::new([]), Rarity::All).set_fractured(true)
    }

    pub fn six_links() -> Self {
        Self::new("Six Linked Bases", &[], Box::new([]), Rarity::All).set_links(6)
    }

    fn new(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self {
            hide: false,
            name,
            classes,
            items,
            rarity,
            map_tier: 0,
            area_level: 1,
            fractured: false,
            influenced: false,
            synthesized: false,
            links: 0,
            text_color: color::WHITE,
            bg_color: color::BLACK,
            outline_color: color::TRANSPARENT,
            font_size: 24,
            effect: Effect::NONE,
            finalize: true,
        }
    }

    fn set_color(self, text_color: Color, bg_color: Color, outline_color: Color) -> Self {
        Self {
            text_color,
            bg_color,
            outline_color,
            ..self
        }
    }

    fn set_effect(self, effect: Effect) -> Self {
        Self { effect, ..self }
    }

    fn set_map_tier(self, map_tier: u8) -> Self {
        Self { map_tier, ..self }
    }

    fn set_influenced(self, influenced: bool) -> Self {
        Self { influenced, ..self }
    }

    fn set_fractured(self, fractured: bool) -> Self {
        Self { fractured, ..self }
    }

    fn set_synthesized(self, synthesized: bool) -> Self {
        Self {
            synthesized,
            ..self
        }
    }

    fn set_links(self, links: u8) -> Self {
        Self { links, ..self }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let show_or_hide = if self.hide {
            format!("Hide # {}", self.name)
        } else {
            format!("Show # {}", self.name)
        };
        let classes = if self.classes.is_empty() {
            String::new()
        } else {
            format!(
                "BaseType {}",
                self.classes
                    .iter()
                    .map(|class| "\"".to_owned() + class.name + "\" ")
                    .collect::<String>()
                    .trim_end()
            )
        };
        let base_types = if self.items.is_empty() {
            String::new()
        } else {
            format!(
                "BaseType {}",
                self.items
                    .iter()
                    .map(|item| "\"".to_owned() + item.base_type + "\" ")
                    .collect::<String>()
                    .trim_end()
            )
        };
        let rarity = if self.rarity == Rarity::All {
            String::new()
        } else {
            format!("Rarity {}", self.rarity)
        };
        let map_tier = if self.map_tier == 0 {
            String::new()
        } else {
            format!("MapTier >= {}", self.map_tier)
        };
        let area_level = if self.area_level == 1 {
            String::new()
        } else {
            format!("AreaLevel {}", self.area_level)
        };
        let fractured = if self.fractured {
            "FracturedItem True".to_string()
        } else {
            String::new()
        };
        let influenced = if self.influenced {
            "HasInfluence Crusader Elder Hunter Redeemer Shaper Warlord".to_string()
        } else {
            String::new()
        };
        let synthesized = if self.synthesized {
            "SynthesizedItem True".to_string()
        } else {
            String::new()
        };
        let links = if self.links > 0 {
            format!("LinkedSockets {}", self.links)
        } else {
            String::new()
        };
        let font_size = format!("SetFontSize {}", self.font_size);
        let text_color = format!("SetTextColor {}", self.text_color);
        let bg_color = format!("SetBackgroundColor {}", self.bg_color);
        let border_color = format!("SetBorderColor {}", self.outline_color);
        let effect = format!("{}", self.effect);
        let finalize = if self.finalize {
            String::new()
        } else {
            "Continue".to_string()
        };

        if self == &Rule::default() {
            Ok(())
        } else {
            writeln!(
                f,
                "{}",
                [
                    show_or_hide,
                    classes,
                    base_types,
                    rarity,
                    map_tier,
                    area_level,
                    fractured,
                    influenced,
                    synthesized,
                    links,
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    effect,
                    finalize,
                ]
                .into_iter()
                .filter(|line| !line.is_empty())
                .map(|line| line + "\n")
                .collect::<String>(),
            )
        }
    }
}
