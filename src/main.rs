use std::fs;

use filter_maker::{
    environment::config::Config,
    loot_filter::{filter::Filter, rarity::Rarity, rule::Rule},
};

fn main() {
    let config: Config = Config::new();
    fs::write(
        "./FilterMaker.filter",
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
            .chain(Rule::hide())
            .collect::<Vec<Rule>>(),
        )
        .to_string()
        .trim_end(),
    )
    .expect("Could not write to FilterMaker.filter file.");
}
