use crate::{
    environment::config::Config,
    loot_filter::{rarity::Rarity, rule::Rule},
};
use std::{fmt::Display, fs};

/// [`Filter`] represents the entire item filter.
#[derive(Default)]
pub struct Filter {
    /// [`Filter::rules`] is a collection of [`Rules`](Rule) that makes up a filter.
    pub rules: Vec<Rule>,
}

/// Implement [`Filter`].
impl Filter {
    /// [`Filter::save`] will create and write a new [`Filter`] based on configuration
    /// values defined in the `.cargo/config.toml` configuration file.
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
                    Rule::fractured(config.show_fractured),
                    Rule::influenced(config.show_influenced),
                    Rule::synthesized(config.show_synthesized),
                    Rule::six_links(config.show_six_links),
                    Rule::hide_uniques(config.hide_uniques),
                ]
                .into_iter()
                .chain(Rule::maps(config.map_tier))
                .chain(Rule::gold(config.show_gold, config.show_gold_icons))
                .chain(Rule::base_styles(config.show_quality_over))
                .chain(Rule::hide())
                .collect::<Vec<Rule>>(),
            )
            .to_string()
            .trim_end(),
        )
        .is_ok()
    }

    /// [`Filter::new`] will create a new [`Filter`].
    fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

/// Implement [`Display`] for [`Filter`].
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
