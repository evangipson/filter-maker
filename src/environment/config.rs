use crate::loot_filter::item::Item;

#[derive(Default)]
pub struct Config {
    pub schwings: Box<[Item]>,
    pub dings: Box<[Item]>,
    pub pings: Box<[Item]>,
    pub uniques: Box<[Item]>,
    pub map_tier: u8,
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
        }
    }
}
