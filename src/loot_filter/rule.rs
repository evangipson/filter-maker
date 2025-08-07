use crate::loot_filter::{
    class::Class,
    color::{self, Color},
    effect::Effect,
    item::Item,
    rarity::Rarity,
};
use std::fmt::Display;

pub struct Rule {
    pub hide: bool,
    pub name: &'static str,
    pub classes: &'static [Class],
    pub items: &'static [Item],
    pub rarity: Rarity,
    pub area_level: u8,
    pub text_color: Color,
    pub bg_color: Color,
    pub outline_color: Color,
    pub font_size: u8,
    pub effect: Effect,
    pub finalize: bool,
}

impl Rule {
    pub const SHOW_ALL_MAGIC_SWORDS: Rule = Rule::normal(
        "Show all magic swords",
        &[],
        &[Item::ONE_HAND_SWORD, Item::TWO_HAND_SWORD],
        Rarity::Magic,
    );

    pub const SHOW_ALL_SUPER_VALUABLES: Rule = Rule::schwing(
        "Show all super valuable drops",
        &[],
        &Item::SUPER_VALUABLES,
        Rarity::All,
    );

    const fn normal(
        name: &'static str,
        classes: &'static [Class],
        items: &'static [Item],
        rarity: Rarity,
    ) -> Self {
        Self {
            hide: false,
            name,
            classes,
            items,
            rarity,
            area_level: 1,
            text_color: color::WHITE,
            bg_color: color::BLACK,
            outline_color: color::TRANSPARENT,
            font_size: 24,
            effect: Effect::NONE,
            finalize: true,
        }
    }

    const fn schwing(
        name: &'static str,
        classes: &'static [Class],
        items: &'static [Item],
        rarity: Rarity,
    ) -> Self {
        Self {
            hide: false,
            name,
            classes,
            items,
            rarity,
            area_level: 1,
            text_color: color::RED,
            bg_color: color::WHITE,
            outline_color: color::RED,
            font_size: 48,
            effect: Effect::BIG_DROP,
            finalize: true,
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("######\n# {}\n######", self.name);
        let show_or_hide = (if self.hide { "Hide" } else { "Show" }).to_string();
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
        let area_level = if self.area_level == 1 {
            String::new()
        } else {
            format!("AreaLevel {}", self.area_level)
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

        writeln!(
            f,
            "{}",
            [
                name,
                show_or_hide,
                classes,
                base_types,
                rarity,
                area_level,
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
