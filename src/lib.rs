//! # Filter Maker
//! [`filter-maker`](crate) is an item filter generator for Path of Exile.
//!
//! ## Getting Started
//! 1. Use `cargo run` or run the `update.ps1` script to generate your filter
//! 1. Edit `.cargo/config.toml` however you see fit
//! 1. Use `cargo run` or run the `update.ps1` script to regenerate your filter

/// [`alerts`] is a collection of functionality for filter alerts.
pub mod alerts {
    /// [`beam`] holds all functionality for showing beams on the minimap.
    pub mod beam;
    /// [`color`] holds all functionality for colors used in alerts.
    pub mod color;
    /// [`icon`] holds all functionality for showing icons on the minimap.
    pub mod icon;
    /// [`icon_size`] holds all functionality for sizing an icon on the minimap.
    pub mod icon_size;
    /// [`icon_type`] holds all functionality for different icon shapes on the minimap.
    pub mod icon_type;
    /// [`name`] holds a map of names for different alert sounds.
    pub mod name;
    /// [`sound`] holds all functionality for creating alert sounds.
    pub mod sound;
    /// [`volume`] holds all functionality for volumes for alerts.
    pub mod volume;
}

/// [`environment`] is a collection of functionality for environmental configuration.
pub mod environment {
    /// [`config`] holds all functionality for filter configuration.
    pub mod config;
}

/// [`loot_filter`] is a collection of functionality for item filters.
pub mod loot_filter {
    /// [`class`] holds all functionality for item classes.
    pub mod class;
    /// [`conditional`] is a trait for conditionally returning default values.
    pub mod conditional;
    /// [`custom_color`] holds all functionality related to dynamic RGBA colors.
    pub mod custom_color;
    /// [`effect`] holds all functionality related to effects for dropped items.
    pub mod effect;
    /// [`filter`] holds all functionality for the item filter itself.
    pub mod filter;
    /// [`item`] holds all functionality for specific items.
    pub mod item;
    /// [`rarity`] holds all functionality for item rarity.
    pub mod rarity;
    /// [`rule`] holds all functionality for individual rules for the item filter.
    pub mod rule;
}
