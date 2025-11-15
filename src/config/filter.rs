use crate::config::{base_config::BaseConfig, rule::Rule};
use serde_derive::Deserialize;
use std::{fs, process::exit};

/// [`Filter`] is a rust representation of the whole TOML filter configuration file.
#[derive(Default, Deserialize, PartialEq)]
pub struct Filter {
    /// [`Filter::configs`] is the top-level `[configs]` portion of the TOML config file.
    pub configs: BaseConfig,

    /// [`Filter::filters`] is a collection of all the `[[filters]]` in the TOML config file.
    pub filters: Vec<Rule>,
}

/// Implement [`Filter`].
impl Filter {
    /// [`Filter::load_config`] will load the TOML configuration file from `filter_path`, and
    /// deserialize it into a [`Filter`].
    /// # Example
    /// You can use [`Filter::load_config`] to load a TOML configuration file:
    /// ```rust
    /// use filter_maker::config::filter::Filter;
    ///
    /// fn create_filter() -> Filter {
    ///     Filter::load_config("config/filter.example.toml")
    /// }
    /// ```
    pub fn load_config(filter_path: &str) -> Self {
        let contents = match fs::read_to_string(filter_path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read theme configuration file `{filter_path}`");
                exit(1)
            }
        };
        toml::from_str(&contents).unwrap()
    }

    /// [`Filter::get_filter`] will generate item filter rules for all the styles and rules
    /// in [`Filter::filters`], then give it back as a [`String`].
    /// # Example
    /// You can use [`Filter::get_filter`] to get all the styles and rules from a [`Filter`]:
    /// ```rust
    /// use filter_maker::config::filter::Filter;
    ///
    /// fn get_filter_rules() -> String {
    ///     Filter::load_config("config/filter.example.toml").get_filter()
    /// }
    /// ```
    pub fn get_filter(&self) -> String {
        format!(
            "{}{}",
            self.configs
                .styles
                .iter()
                .map(|s| format!(
                    "{}\n",
                    s.get_style(self.configs.palette.clone(), &self.configs.mods)
                ))
                .collect::<String>(),
            self.filters
                .iter()
                .map(|f| format!(
                    "{}\n",
                    f.get_rule(self.configs.palette.clone(), &self.configs.mods)
                ))
                .collect::<String>()
        )
    }
}
