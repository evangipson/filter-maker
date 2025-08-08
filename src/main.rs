use filter_maker::{
    environment::config::Config,
    loot_filter::{filter::Filter, rarity::Rarity, rule::Rule},
};

fn main() {
    let config: Config = Config::new();
    print!(
        "{}",
        Filter::new(vec![
            Rule::schwing("Schwings (S-Tier)", &[], config.schwings, Rarity::All),
            Rule::ding("Dings (A-Tier)", &[], config.dings, Rarity::All),
            Rule::ping("Pings (B-Tier)", &[], config.pings, Rarity::All),
            Rule::schwing("Uniques (Tier 0)", &[], config.uniques, Rarity::Unique),
            Rule::maps("Maps", config.map_tier, Rarity::All),
            config
                .show_fractured_bases
                .then(Rule::fractured)
                .unwrap_or_default(),
            config
                .show_influenced_bases
                .then(Rule::influenced)
                .unwrap_or_default(),
            config
                .show_synthesized_bases
                .then(Rule::synthesized)
                .unwrap_or_default(),
            config
                .show_six_link_bases
                .then(Rule::six_links)
                .unwrap_or_default(),
        ])
    );
}
