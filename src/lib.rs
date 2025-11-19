//! # Filter Maker
//! [`filter_maker`](crate) is an item filter generator for Path of Exile.
//!
//! ## Getting Started
//! 1. Use `cargo run -- [PATH_TO_FILTER] [PATH_TO_DESTINATION]` to regenerate
//!    your filter
//! 1. Optionally edit the filter.poe1.toml configuration file to customize
//!    the filter

/// [`config`] is a collection of functionality to allow filter configuration.
pub mod config {
    /// [`base_config`] is a rust representation of the `[configs]` section of
    /// the TOML filter configuration file.
    pub mod base_config;
    /// [`color`] is a rust representation of a color defined in the TOML
    /// filter configuration file.
    pub mod color;
    /// [`filter`] is a rust representation of the whole TOML filter
    /// configuration file.
    pub mod filter;
    /// [`icon`] is a rust representation of an icon defined in the TOML
    /// configuration file.
    pub mod icon;
    /// [`modifier`] is a rust representation of an item modifier defined in
    /// the TOML configuration file.
    pub mod modifier;
    /// [`rule`] is a rust representation of a filter rule defined in the TOML
    /// configuration file.
    pub mod rule;
    /// [`sound`] is a rust representation of a sound defined in the TOML
    /// configuration file.
    pub mod sound;
    /// [`style`] is a rust representation of a base style filter rule defined
    /// in the TOML configuration file.
    pub mod style;
    /// [`theme`] is a rust representation of a visual theme defined in the
    /// TOML configuration file.
    pub mod theme;
}

/// [`behavior`] is a collection of functionality to define behaviors.
pub mod behavior {
    /// [`common`] is a collection of commonly used functions throughout
    /// [`filter_maker`](crate).
    pub mod common;
    /// [`conditional`] is a trait for conditionally returning default values.
    pub mod conditional;
    /// [`write_rules`] is a trait for writing out rules to the filter.
    pub mod write_rules;
}

/// [`color`] is a collection of functionality to define colors.
pub mod color {
    /// [`custom_color`] holds all functionality related to dynamic RGBA colors.
    pub mod custom_color;
}

/// [`constants`] is a collection of constant values used in various places.
pub mod constants {
    /// [`filter`] is a collection of constant values used for the item filter.
    pub mod filter;
    /// [`rules`] is a collection of constant values used for rules.
    pub mod rules;
}

/// [`os`] is a collection of functionality for operation systems.
pub mod os {
    /// [`copy_file`] holds all functionality related to copying files.
    pub mod copy_file;
}
