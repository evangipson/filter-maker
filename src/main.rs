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
            Rule::maps(config.map_tier),
            Rule::fractured(config.show_fractured),
            Rule::influenced(config.show_influenced),
            Rule::synthesized(config.show_synthesized),
            Rule::six_links(config.show_six_links),
        ])
    );
}
