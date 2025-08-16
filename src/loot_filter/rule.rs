use crate::loot_filter::{
    class::Class, conditional::Conditional, custom_color::CustomColor, effect::Effect, item::Item,
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
    pub text_color: CustomColor,
    pub bg_color: CustomColor,
    pub outline_color: CustomColor,
    pub font_size: u8,
    pub effect: Effect,
    pub finalize: bool,
    pub strict: bool,
}

impl Rule {
    pub const FRACTURED: &str = "FracturedItem True";
    pub const INFLUENCED: &str = "HasInfluence Crusader Elder Hunter Redeemer Shaper Warlord";
    pub const SYNTHESIZED: &str = "SynthesisedItem True";

    pub fn schwing(
        name: &'static str,
        classes: &'static [Class],
        items: Box<[Item]>,
        rarity: Rarity,
    ) -> Self {
        Self::new(name, classes, items, rarity)
            .set_color(
                CustomColor::DIVINE_RED,
                CustomColor::NORMAL_WHITE,
                CustomColor::DIVINE_RED,
            )
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
                CustomColor::DARK_MAGIC_BLUE,
                CustomColor::NORMAL_WHITE,
                CustomColor::DARK_MAGIC_BLUE,
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
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::PURPLE,
                CustomColor::NORMAL_WHITE,
            )
    }

    pub fn dust_uniques(uniques: Box<[Item]>) -> Self {
        Self::new("Uniques (dust)", &[], uniques, Rarity::Unique)
            .set_font_size(38)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::DARK_PINK,
                CustomColor::NORMAL_WHITE,
            )
            .set_effect(Effect::INTERESTING_DROP)
            .set_strict(false)
    }

    pub fn hide_uniques(uniques: Box<[Item]>) -> Self {
        Self::new("Uniques (hide)", &[], uniques, Rarity::Unique)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::UNIQUE_ORANGE,
                CustomColor::TRANSPARENT,
            )
            .set_font_size(26)
            .set_hide(true)
    }

    pub fn maps(map_tier: u8) -> Vec<Rule> {
        vec![
            Self::new("Maps (tier 17)", &[Class::MAPS], Box::new([]), Rarity::None)
                .set_font_size(38)
                .set_color(
                    CustomColor::PURPLE,
                    CustomColor::NORMAL_WHITE,
                    CustomColor::PURPLE,
                )
                .set_map_tier(17)
                .set_effect(Effect::INTERESTING_DROP),
            Self::new(
                "Maps (whisper)",
                &[Class::MEMORY],
                Box::new([]),
                Rarity::None,
            )
            .set_font_size(28)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::DARK_PURPLE,
                CustomColor::NORMAL_WHITE,
            )
            .set_effect(Effect::MID_DROP),
            Self::new("Maps", &[Class::MAPS], Box::new([]), Rarity::None)
                .set_font_size(32)
                .set_color(
                    CustomColor::NORMAL_WHITE,
                    CustomColor::BLACK,
                    CustomColor::TRANSPARENT,
                )
                .set_map_tier(map_tier)
                .set_effect(Effect::SMALL_DROP)
                .only_if(map_tier < 17),
        ]
    }

    pub fn influenced(show_rule: bool) -> Self {
        Self::new("Influenced Bases", &[], Box::new([]), Rarity::None)
            .set_font_size(38)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::QUEST_GREEN,
                CustomColor::NORMAL_WHITE,
            )
            .set_influenced(true)
            .set_effect(Effect::INTERESTING_DROP)
            .only_if(show_rule)
    }

    pub fn synthesized(show_rule: bool) -> Self {
        Self::new("Synthesized Bases", &[], Box::new([]), Rarity::All)
            .set_font_size(38)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::SYNTHESIZED_PINK,
                CustomColor::NORMAL_WHITE,
            )
            .set_synthesized(true)
            .set_effect(Effect::SPECIAL_DROP)
            .only_if(show_rule)
    }

    pub fn fractured(show_rule: bool) -> Self {
        Self::new("Fractured Bases", &[], Box::new([]), Rarity::None)
            .set_font_size(38)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::GEM_TEAL,
                CustomColor::NORMAL_WHITE,
            )
            .set_fractured(true)
            .set_effect(Effect::INTERESTING_DROP)
            .only_if(show_rule)
    }

    pub fn six_links(show_rule: bool) -> Self {
        Self::new("Six Linked Bases", &[], Box::new([]), Rarity::None)
            .set_font_size(36)
            .set_links(6)
            .set_effect(Effect::LINKED_DROP)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::DIVINE_RED,
                CustomColor::NORMAL_WHITE,
            )
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
            .set_color(
                CustomColor::KALGUUR_GOLD,
                CustomColor::BLACK,
                CustomColor::KALGUUR_GOLD,
            )
            .set_effect(Effect::GOLD_PILE),
            Self::new(
                "Gold (large pile)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(350)
            .set_font_size(38)
            .set_color(
                CustomColor::KALGUUR_GOLD,
                CustomColor::FADED_BLACK,
                CustomColor::KALGUUR_GOLD,
            ),
            Self::new(
                "Gold (medium pile)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(200)
            .set_font_size(32)
            .set_color(
                CustomColor::KALGUUR_GOLD,
                CustomColor::FADED_BLACK,
                CustomColor::KALGUUR_GOLD,
            ),
            Self::new(
                "Gold (small pile)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(100)
            .set_font_size(24)
            .set_color(
                CustomColor::KALGUUR_GOLD,
                CustomColor::FADED_BLACK,
                CustomColor::TRANSPARENT,
            ),
            Self::new(
                "Gold (tiny piles)",
                &[Class::CURRENCY],
                Box::new([gold_item]),
                Rarity::None,
            )
            .set_stack_size(1)
            .set_font_size(22)
            .set_color(
                CustomColor::KALGUUR_GOLD,
                CustomColor::FADED_BLACK,
                CustomColor::TRANSPARENT,
            ),
        ]
    }

    pub fn base_styles() -> Vec<Rule> {
        vec![
            Self::new(
                "Breach Splinters (base style)",
                &[],
                Box::new([Item::new("Splinter of")]),
                Rarity::None,
            )
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::DIVINE_RED,
                CustomColor::DARK_PURPLE,
            )
            .set_font_size(28)
            .set_strict(false),
            Self::new(
                "Scarabs (base style)",
                &[Class::BREACHSTONES, Class::MAP_FRAGMENTS],
                Box::new([Item::new("Scarab")]),
                Rarity::All,
            )
            .set_color(
                CustomColor::GEM_TEAL,
                CustomColor::NORMAL_WHITE,
                CustomColor::GEM_TEAL,
            )
            .set_font_size(32)
            .set_strict(false),
            Self::new(
                "Delirium Orb (base style)",
                &[],
                Box::new([Item::new("Delirium Orb")]),
                Rarity::None,
            )
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::FADED_CURRENCY_SALMON,
                CustomColor::CURRENCY_SALMON,
            )
            .set_font_size(28)
            .set_strict(false),
            Self::new(
                "Gems (base style)",
                &[Class::SKILL_GEMS, Class::SUPPORT_GEMS],
                Box::new([Item::new("Gem")]),
                Rarity::All,
            )
            .set_color(
                CustomColor::GEM_TEAL,
                CustomColor::FADED_BLACK,
                CustomColor::GEM_TEAL,
            )
            .set_font_size(32)
            .set_strict(false),
            Self::new(
                "Maps (synthesised)",
                &[],
                Box::new([Item::new("Synthesised Map")]),
                Rarity::None,
            )
            .set_font_size(32)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::PURPLE,
                CustomColor::NORMAL_WHITE,
            )
            .set_effect(Effect::INTERESTING_DROP),
            Self::new(
                "Maps (base 'normal' style)",
                &[Class::MAPS],
                Box::new([]),
                Rarity::Normal,
            )
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::FADED_BLACK,
                CustomColor::NORMAL_WHITE,
            )
            .set_font_size(32),
            Self::new(
                "Maps (base 'magic' style)",
                &[Class::MAPS],
                Box::new([]),
                Rarity::Magic,
            )
            .set_color(
                CustomColor::MAGIC_BLUE,
                CustomColor::FADED_BLACK,
                CustomColor::MAGIC_BLUE,
            )
            .set_font_size(32),
            Self::new(
                "Maps (base 'rare' style)",
                &[Class::MAPS],
                Box::new([]),
                Rarity::Rare,
            )
            .set_color(
                CustomColor::RARE_YELLOW,
                CustomColor::FADED_BLACK,
                CustomColor::RARE_YELLOW,
            )
            .set_font_size(32),
            Self::new(
                "Maps (base 'unique' style)",
                &[Class::MAPS],
                Box::new([]),
                Rarity::Unique,
            )
            .set_color(
                CustomColor::UNIQUE_ORANGE,
                CustomColor::FADED_BLACK,
                CustomColor::UNIQUE_ORANGE,
            )
            .set_font_size(32),
        ]
    }

    pub fn hide() -> Vec<Rule> {
        vec![
            Self::new(
                "All scrolls",
                &[],
                Box::new([Item::new("Scroll of Wisdom"), Item::new("Portal Scroll")]),
                Rarity::None,
            )
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::FADED_MAGIC_BLUE,
                CustomColor::DARK_MAGIC_BLUE,
            )
            .set_font_size(22)
            .set_hide(true),
            Self::new(
                "All remaining divination cards",
                &[Class::DIVINATION_CARDS],
                Box::new([]),
                Rarity::None,
            )
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::DARK_MAGIC_BLUE,
                CustomColor::NORMAL_WHITE,
            )
            .set_font_size(22)
            .set_hide(true),
            Self::new(
                "All remaining currency",
                &[Class::CURRENCY],
                Box::new([]),
                Rarity::None,
            )
            .set_font_size(22)
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::FADED_CURRENCY_SALMON,
                CustomColor::CURRENCY_SALMON,
            )
            .set_hide(true),
            Self::new(
                "All normal items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Normal,
            )
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::FADED_BLACK,
                CustomColor::TRANSPARENT,
            )
            .set_font_size(16)
            .set_hide(true),
            Self::new(
                "All magic items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Magic,
            )
            .set_color(
                CustomColor::MAGIC_BLUE,
                CustomColor::FADED_BLACK,
                CustomColor::TRANSPARENT,
            )
            .set_font_size(16)
            .set_hide(true),
            Self::new(
                "All rare items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Rare,
            )
            .set_color(
                CustomColor::RARE_YELLOW,
                CustomColor::FADED_BLACK,
                CustomColor::TRANSPARENT,
            )
            .set_font_size(16)
            .set_hide(true),
            Self::new(
                "All unique items",
                &Class::ALL_CLASSES,
                Box::new([]),
                Rarity::Unique,
            )
            .set_color(
                CustomColor::NORMAL_WHITE,
                CustomColor::UNIQUE_ORANGE,
                CustomColor::TRANSPARENT,
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

    fn set_color(
        self,
        text_color: CustomColor,
        bg_color: CustomColor,
        outline_color: CustomColor,
    ) -> Self {
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
