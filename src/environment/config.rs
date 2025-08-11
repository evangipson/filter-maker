use crate::loot_filter::item::Item;

#[derive(Default)]
pub struct Config {
    pub schwings: Box<[Item]>,
    pub dings: Box<[Item]>,
    pub pings: Box<[Item]>,
    pub uniques: Box<[Item]>,
    pub dust_uniques: Box<[Item]>,
    pub hide_uniques: Box<[Item]>,
    pub map_tier: u8,
    pub show_gold: bool,
    pub show_fractured: bool,
    pub show_influenced: bool,
    pub show_synthesized: bool,
    pub show_six_links: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            schwings: Self::get_items(env!("schwings")).into(),
            dings: Self::get_items(env!("dings")).into(),
            pings: Self::get_items(env!("pings")).into(),
            uniques: Self::get_items(env!("uniques")).into(),
            dust_uniques: Self::get_items(env!("dust_uniques")).into(),
            hide_uniques: Self::get_items(env!("hide_uniques")).into(),
            map_tier: Self::get_number(env!("map_tier")),
            show_gold: Self::get_boolean(env!("show_gold")),
            show_fractured: Self::get_boolean(env!("show_fractured_bases")),
            show_influenced: Self::get_boolean(env!("show_influenced_bases")),
            show_synthesized: Self::get_boolean(env!("show_synthesized_bases")),
            show_six_links: Self::get_boolean(env!("show_six_link_bases")),
        }
    }

    fn get_items(items: &'static str) -> Vec<Item> {
        items
            .lines()
            .map(|item_name| Item::new(item_name.trim()))
            .collect::<Vec<Item>>()
    }

    fn get_boolean(value: &'static str) -> bool {
        value.parse::<bool>().unwrap_or_default()
    }

    fn get_number(value: &'static str) -> u8 {
        value.parse::<u8>().unwrap_or_default()
    }
}
