use crate::{
    environment::config::Config,
    loot_filter::{rarity::Rarity, rule::Rule},
};
use std::{fmt::Display, fs};

#[derive(Default)]
pub struct Filter {
    pub rules: Vec<Rule>,
}

impl Filter {
    pub fn save() -> bool {
        let config = Config::new();
        fs::write(
            config.destination,
            Filter::new(
                vec![
                    Rule::schwing("Schwings (S-Tier)", &[], config.schwings, Rarity::None),
                    Rule::ding("Dings (A-Tier)", &[], config.dings, Rarity::None),
                    Rule::ping("Pings (B-Tier)", &[], config.pings, Rarity::None),
                    Rule::schwing("Uniques (Tier 0)", &[], config.uniques, Rarity::Unique),
                    Rule::dust_uniques(config.dust_uniques),
                    Rule::hide_uniques(config.hide_uniques),
                    Rule::fractured(config.show_fractured),
                    Rule::influenced(config.show_influenced),
                    Rule::synthesized(config.show_synthesized),
                    Rule::six_links(config.show_six_links),
                ]
                .into_iter()
                .chain(Rule::maps(config.map_tier))
                .chain(Rule::gold(config.show_gold))
                .chain(Rule::base_styles())
                .chain(Rule::hide())
                .collect::<Vec<Rule>>(),
            )
            .to_string()
            .trim_end(),
        )
        .is_ok()
    }

    fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.rules
                .iter()
                .map(|rule| rule.to_string())
                .collect::<String>()
        )
    }
}
