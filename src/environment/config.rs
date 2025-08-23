use crate::loot_filter::item::Item;

/// [`Config`] represents a collection of configuration values from the `.cargo/config.toml`
/// configuration file.
#[derive(Default)]
pub struct Config {
    /// [`Config::destination`] represents the folder destination and name of the generated
    /// filter.
    pub destination: String,
    /// [`Config::schwings`] represents a collection of items that will receive the most visual
    /// attention from the filter ("S"-tier).
    pub schwings: Box<[Item]>,
    /// [`Config::dings`] represents a collection of items that will receive moderate visual
    /// attention from the filter ("A"-tier).
    pub dings: Box<[Item]>,
    /// [`Config::pings`] represents a collection of items that will receive minor visual
    /// attention from the filter ("B"-tier).
    pub pings: Box<[Item]>,
    /// [`Config::uniques`] represents a collection of unique items that will receive the most
    /// visual attention from the filter ("Tier 0" uniques).
    pub uniques: Box<[Item]>,
    /// [`Config::dust_uniques`] represents a collection of unique items that will be highlighted
    /// using a unique color scheme ("Tier 1" uniques).
    pub dust_uniques: Box<[Item]>,
    /// [`Config::hide_uniques`] represents a collection of unique items that will be hidden by
    /// the filter.
    pub hide_uniques: Box<[Item]>,
    /// [`Config::map_tier`] represents the tiers of maps that will be shown by the filter.
    pub map_tier: u8,
    /// [`Config::show_gold`] represents a setting to show progressively larger gold piles based
    /// on amount.
    pub show_gold: bool,
    /// [`Config::show_fractured`] represents a setting to show all fractured items.
    pub show_fractured: bool,
    /// [`Config::show_influenced`] represents a setting to show all influenced items.
    pub show_influenced: bool,
    /// [`Config::show_synthesized`] represents a setting to show all synthesized items.
    pub show_synthesized: bool,
    /// [`Config::show_six_links`] represents a setting to show all six-linked items.
    pub show_six_links: bool,
    /// [`Config::show_gold_icons`] represents a setting to show minimap icons for all gold.
    pub show_gold_icons: bool,
    /// [`Config::show_gold_icons`] represents a setting for the item quality required for the filter to display it.
    pub show_quality_over: u8,
}

/// Implement [`Config`].
impl Config {
    /// [`Config::new`] will create a new [`Config`] based on configuration values in the
    /// `.cargo/config.toml` configuration file.
    pub fn new() -> Self {
        Self {
            destination: env!("destination").to_string(),
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
            show_gold_icons: Self::get_boolean(env!("show_gold_icons")),
            show_quality_over: Self::get_number(env!("show_quality_over")),
        }
    }

    /// [`Config::get_items`] will get a collection of [`Item`] from a multi-line string
    /// in the configuration file.
    fn get_items(items: &'static str) -> Vec<Item> {
        items
            .lines()
            .map(|item_name| Item::new(item_name.trim()))
            .collect::<Vec<Item>>()
    }

    /// [`Config::get_boolean`] will get a [`bool`] from a string in the configuration file.
    fn get_boolean(value: &'static str) -> bool {
        value.parse::<bool>().unwrap_or_default()
    }

    /// [`Config::get_number`] will get a [`u8`] from a string in the configuration file.
    fn get_number(value: &'static str) -> u8 {
        value.parse::<u8>().unwrap_or_default()
    }
}
