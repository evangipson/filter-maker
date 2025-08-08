use crate::loot_filter::item::Item;

#[derive(Default)]
pub struct Config {
    pub schwings: Box<[Item]>,
    pub dings: Box<[Item]>,
    pub pings: Box<[Item]>,
    pub uniques: Box<[Item]>,
    pub map_tier: u8,
    pub show_gold: bool,
    pub show_fractured_bases: bool,
    pub show_influenced_bases: bool,
    pub show_synthesized_bases: bool,
    pub show_six_link_bases: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            schwings: env!("schwings")
                .lines()
                .map(|item_name| Item::new(item_name.trim()))
                .collect::<Vec<Item>>()
                .into(),
            dings: env!("dings")
                .lines()
                .map(|item_name| Item::new(item_name.trim()))
                .collect::<Vec<Item>>()
                .into(),
            pings: env!("pings")
                .lines()
                .map(|item_name| Item::new(item_name.trim()))
                .collect::<Vec<Item>>()
                .into(),
            uniques: env!("uniques")
                .lines()
                .map(|item_name| Item::new(item_name.trim()))
                .collect::<Vec<Item>>()
                .into(),
            map_tier: env!("map_tier").parse::<u8>().unwrap_or_default(),
            show_gold: env!("show_gold").parse::<bool>().unwrap_or_default(),
            show_fractured_bases: env!("show_fractured_bases")
                .parse::<bool>()
                .unwrap_or_default(),
            show_influenced_bases: env!("show_influenced_bases")
                .parse::<bool>()
                .unwrap_or_default(),
            show_synthesized_bases: env!("show_synthesized_bases")
                .parse::<bool>()
                .unwrap_or_default(),
            show_six_link_bases: env!("show_six_link_bases")
                .parse::<bool>()
                .unwrap_or_default(),
        }
    }
}
