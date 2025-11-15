//! # Filter Maker
//! [`filter-maker`](crate) is an item filter generator for Path of Exile.
//!
//! ## Getting Started
//! 1. Use `cargo run` or run the `update.ps1` script to generate your filter
//! 1. Edit your filter configuration TOML file however you see fit
//! 1. Use `cargo run -- [PATH_TO_FILTER]` or run the `update.ps1` script to regenerate your filter

/// [`config`] is a collection of functionality to allow filter configuration.
pub mod config {
    pub mod base_config;
    pub mod color;
    pub mod filter;
    pub mod icon;
    pub mod modifier;
    pub mod rule;
    pub mod sound;
    pub mod style;
    pub mod theme;
}

/// [`behavior`] is a collection of functionality to define behaviors.
pub mod behavior {
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
    /// [`rules`] is a collection of constant values used for rules.
    pub mod rules;
}

/// [`os`] is a collection of functionality for operation systems.
pub mod os {
    /// [`copy_file`] holds all functionality related to copying files.
    pub mod copy_file;
    /// [`find_directory`] holds all functionality related to finding directories.
    pub mod find_directory;
}
