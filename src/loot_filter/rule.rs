use crate::loot_filter::{
    class::Class,
    color::{self, Color},
    conditional::Conditional,
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
    pub stack_size: u16,
    pub text_color: Color,
    pub bg_color: Color,
    pub outline_color: Color,
    pub font_size: u8,
    pub effect: Effect,
    pub finalize: bool,
    pub strict: bool,
}

impl Rule {
    pub const FRACTURED: &str = "FracturedItem True";
    pub const INFLUENCED: &str = "HasInfluence Crusader Elder Hunter Redeemer Shaper Warlord";
    pub const SYNTHESIZED: &str = "SynthesisedItem True";
    pub const DEFAULT_FONT_SIZE: u8 = 28;

    pub fn schwing(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self::new(name, classes, items, rarity)
            .set_color(color::DIVINE_RED, color::NORMAL_WHITE, color::DIVINE_RED)
            .set_font_size(48)
            .set_effect(Effect::BIG_DROP)
    }

    pub fn ding(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self::new(name, classes, items, rarity)
            .set_color(
                color::DARK_MAGIC_BLUE,
                color::NORMAL_WHITE,
                color::DARK_MAGIC_BLUE,
            )
            .set_font_size(45)
            .set_effect(Effect::NORMAL_DROP)
    }

    pub fn ping(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self::new(name, classes, items, rarity)
            .set_font_size(42)
            .set_color(color::NORMAL_WHITE, color::PURPLE, color::NORMAL_WHITE)
    }

    pub fn dust_uniques(uniques: Box<[Item]>) -> Self {
        Self::new("Uniques (dust)", &[], uniques, Rarity::Unique)
            .set_font_size(38)
            .set_color(color::NORMAL_WHITE, color::DARK_PINK, color::NORMAL_WHITE)
            .set_effect(Effect::INTERESTING_DROP)
    }

    pub fn hide_uniques(uniques: Box<[Item]>) -> Self {
        Self::new("Uniques (hide)", &[], uniques, Rarity::Unique)
            .set_color(
                color::NORMAL_WHITE,
                color::UNIQUE_ORANGE,
                color::TRANSPARENT,
            )
            .set_font_size(26)
            .set_hide(true)
    }

    pub fn maps(map_tier: u8) -> Vec<Rule> {
        vec![
            Self::new("Maps", &[Class::MAPS], Box::new([]), Rarity::None)
                .set_font_size(32)
                .set_color(color::NORMAL_WHITE, color::BLACK, color::TRANSPARENT)
                .set_map_tier(map_tier)
                .set_effect(Effect::SMALL_DROP)
                .only_if(map_tier < 17),
            Self::new("Maps (tier 17)", &[Class::MAPS], Box::new([]), Rarity::None)
                .set_font_size(38)
                .set_color(color::PURPLE, color::NORMAL_WHITE, color::PURPLE)
                .set_map_tier(17)
                .set_effect(Effect::INTERESTING_DROP),
        ]
    }

    pub fn influenced(show_rule: bool) -> Self {
        Self::new("Influenced Bases", &[], Box::new([]), Rarity::None)
            .set_font_size(38)
            .set_color(color::NORMAL_WHITE, color::QUEST_GREEN, color::NORMAL_WHITE)
            .set_influenced(true)
            .set_effect(Effect::INTERESTING_DROP)
            .only_if(show_rule)
    }

    pub fn synthesized(show_rule: bool) -> Self {
        Self::new("Synthesized Bases", &[], Box::new([]), Rarity::None)
            .set_font_size(38)
            .set_color(color::NORMAL_WHITE, color::PURPLE, color::NORMAL_WHITE)
            .set_synthesized(true)
            .set_effect(Effect::INTERESTING_DROP)
            .only_if(show_rule)
    }

    pub fn fractured(show_rule: bool) -> Self {
        Self::new("Fractured Bases", &[], Box::new([]), Rarity::None)
            .set_font_size(38)
            .set_color(color::NORMAL_WHITE, color::GEM_TEAL, color::NORMAL_WHITE)
            .set_fractured(true)
            .set_effect(Effect::INTERESTING_DROP)
            .only_if(show_rule)
    }

    pub fn six_links(show_rule: bool) -> Self {
        Self::new("Six Linked Bases", &[], Box::new([]), Rarity::None)
            .set_font_size(36)
            .set_links(6)
            .set_effect(Effect::LINKED_DROP)
            .set_color(color::NORMAL_WHITE, color::DIVINE_RED, color::NORMAL_WHITE)
            .only_if(show_rule)
    }

    pub fn gold(show_rule: bool) -> Vec<Rule> {
        if !show_rule {
            return vec![];
        }

        let gold_item = Item::new("Gold");
        vec![
            Self::new(
                "Gold (giant pile with beam)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(500)
            .set_font_size(42)
            .set_color(color::KALGUUR_GOLD, color::BLACK, color::KALGUUR_GOLD)
            .set_effect(Effect::GOLD_PILE),
            Self::new(
                "Gold (large pile)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(350)
            .set_font_size(38)
            .set_color(color::KALGUUR_GOLD, color::FADED_BLACK, color::KALGUUR_GOLD),
            Self::new(
                "Gold (medium pile)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(200)
            .set_font_size(32)
            .set_color(color::KALGUUR_GOLD, color::FADED_BLACK, color::KALGUUR_GOLD),
            Self::new(
                "Gold (small pile)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(100)
            .set_font_size(28)
            .set_color(color::KALGUUR_GOLD, color::FADED_BLACK, color::TRANSPARENT),
            Self::new(
                "Gold (tiny piles)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(1)
            .set_font_size(24)
            .set_color(color::KALGUUR_GOLD, color::FADED_BLACK, color::TRANSPARENT),
        ]
    }

    pub fn hide() -> Vec<Rule> {
        vec![
            Self::new(
                "All other scarabs",
                &[Class::BREACHSTONES, Class::MAP_FRAGMENTS],
                Box::new([Item::new("Scarab")]),
                Rarity::All,
            )
            .set_color(color::GEM_TEAL, color::NORMAL_WHITE, color::GEM_TEAL)
            .set_font_size(32)
            .set_strict(false),
            Self::new(
                "All normal items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Normal,
            )
            .set_color(color::NORMAL_WHITE, color::FADED_BLACK, color::TRANSPARENT)
            .set_font_size(16)
            .set_hide(true),
            Self::new(
                "All magic items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Magic,
            )
            .set_color(color::MAGIC_BLUE, color::FADED_BLACK, color::TRANSPARENT)
            .set_font_size(18)
            .set_hide(true),
            Self::new(
                "All rare items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Rare,
            )
            .set_color(color::RARE_YELLOW, color::FADED_BLACK, color::TRANSPARENT)
            .set_font_size(20)
            .set_hide(true),
            Self::new(
                "All unique items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Unique,
            )
            .set_color(
                color::NORMAL_WHITE,
                color::UNIQUE_ORANGE,
                color::TRANSPARENT,
            )
            .set_font_size(24)
            .set_hide(true),
        ]
    }

    fn new(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self {
            name,
            classes,
            items,
            rarity,
            finalize: true,
            strict: true,
            ..Default::default()
        }
    }

    fn set_font_size(self, font_size: u8) -> Self {
        Self { font_size, ..self }
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

    fn set_stack_size(self, stack_size: u16) -> Self {
        Self { stack_size, ..self }
    }

    fn set_hide(self, hide: bool) -> Self {
        Self { hide, ..self }
    }

    fn set_strict(self, strict: bool) -> Self {
        Self { strict, ..self }
    }
}

fn get_display<T: Default + Display + Conditional + PartialEq>(
    prefix: &str,
    property: &T,
) -> String {
    format!("{prefix} {property}").if_not_default(property)
}

fn get_name_display(name: &'static str, hide: bool) -> String {
    format!(
        "{} # {}",
        match hide {
            true => "Hide",
            false => "Show",
        },
        name
    )
}

fn get_item_display(items: &[Item], strict: bool) -> String {
    format!(
        "BaseType {}{}",
        if strict { "== " } else { "" },
        items
            .iter()
            .map(|item| "\"".to_owned() + item.base_type + "\" ")
            .collect::<String>()
            .trim_end()
    )
    .only_if(!items.is_empty())
}

fn get_class_display(classes: &[Class]) -> String {
    format!(
        "Class == {}",
        classes
            .iter()
            .map(|class| "\"".to_owned() + class.name + "\" ")
            .collect::<String>()
            .trim_end()
    )
    .only_if(!classes.is_empty())
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            // if it's a default rule, don't write anything
            Ok(())
        } else {
            writeln!(
                f,
                "{}",
                [
                    get_name_display(self.name, self.hide),
                    get_class_display(self.classes),
                    get_item_display(&self.items, self.strict),
                    get_display("Rarity", &self.rarity),
                    get_display("MapTier >=", &self.map_tier),
                    get_display("AreaLevel", &self.area_level),
                    Rule::FRACTURED.to_string().only_if(self.fractured),
                    Rule::INFLUENCED.to_string().only_if(self.influenced),
                    Rule::SYNTHESIZED.to_string().only_if(self.synthesized),
                    get_display("LinkedSockets", &self.links),
                    get_display("StackSize >=", &self.stack_size),
                    get_display("SetFontSize", &self.font_size),
                    get_display("SetTextColor", &self.text_color),
                    get_display("SetBackgroundColor", &self.bg_color),
                    get_display("SetBorderColor", &self.outline_color),
                    format!("{}", self.effect),
                    "Continue".to_string().except_if(self.finalize),
                ]
                .into_iter()
                .filter(|line| !line.is_empty())
                .map(|line| line + "\n")
                .collect::<String>(),
            )
        }
    }
}
