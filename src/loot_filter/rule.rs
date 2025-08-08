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
}

impl Rule {
    pub const INFLUENCED: &str = "HasInfluence Crusader Elder Hunter Redeemer Shaper Warlord";

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

    pub fn maps(map_tier: u8) -> Self {
        Self::new("Maps", &[], Box::new([]), Rarity::All).set_map_tier(map_tier)
    }

    pub fn influenced(show_rule: bool) -> Self {
        Self::new("Influenced Bases", &[], Box::new([]), Rarity::All)
            .set_influenced(true)
            .only_if(show_rule)
    }

    pub fn synthesized(show_rule: bool) -> Self {
        Self::new("Synthesized Bases", &[], Box::new([]), Rarity::All)
            .set_synthesized(true)
            .only_if(show_rule)
    }

    pub fn fractured(show_rule: bool) -> Self {
        Self::new("Fractured Bases", &[], Box::new([]), Rarity::All)
            .set_fractured(true)
            .only_if(show_rule)
    }

    pub fn six_links(show_rule: bool) -> Self {
        Self::new("Six Linked Bases", &[], Box::new([]), Rarity::All)
            .set_links(6)
            .only_if(show_rule)
    }

    pub fn gold(show_rule: bool) -> Vec<Rule> {
        if !show_rule {
            return vec![];
        }

        let gold_item = Item::new("Gold");
        vec![
            Self::new("Gold (base style)", &[], Box::new([gold_item]), Rarity::All)
                .set_color(color::BLACK, color::YELLOW, color::YELLOW)
                .set_font_size(18)
                .set_finalize(false),
            Self::new("Gold (giant pile)", &[], Box::new([gold_item]), Rarity::All)
                .set_stack_size(1000)
                .set_font_size(32)
                .set_effect(Effect::GOLD_PILE),
            Self::new("Gold (huge pile)", &[], Box::new([gold_item]), Rarity::All)
                .set_stack_size(500)
                .set_font_size(28),
            Self::new("Gold (large pile)", &[], Box::new([gold_item]), Rarity::All)
                .set_stack_size(250)
                .set_font_size(24),
            Self::new("Gold (mid pile)", &[], Box::new([gold_item]), Rarity::All)
                .set_stack_size(100)
                .set_font_size(20),
            Self::new("Gold (small pile)", &[], Box::new([gold_item]), Rarity::All)
                .set_stack_size(1),
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
            text_color: color::WHITE,
            bg_color: color::BLACK,
            font_size: 24,
            finalize: true,
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

    fn set_finalize(self, finalize: bool) -> Self {
        Self { finalize, ..self }
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

fn get_item_display(items: &[Item]) -> String {
    format!(
        "BaseType {}",
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
        "Class {}",
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
                    get_item_display(&self.items),
                    get_display("Rarity", &self.rarity),
                    get_display("MapTier >=", &self.map_tier),
                    get_display("AreaLevel", &self.area_level),
                    get_display("FracturedItem", &self.fractured),
                    get_display(Rule::INFLUENCED, &self.influenced),
                    get_display("SynthesizedItem", &self.synthesized),
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
