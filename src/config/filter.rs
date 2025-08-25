use crate::{
    config::{base_config::BaseConfig, rule::Rule},
    constants::filter_config::FILTER_CONFIG_FILE,
};
use serde_derive::Deserialize;
use std::{fs, process::exit};

#[derive(Default, Deserialize, PartialEq)]
pub struct Filter {
    pub configs: BaseConfig,
    pub rules: Vec<Rule>,
}

impl Filter {
    pub fn load_config() -> Self {
        let contents = match fs::read_to_string(FILTER_CONFIG_FILE) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read theme configuration file `{FILTER_CONFIG_FILE}`");
                exit(1)
            }
        };
        toml::from_str(&contents).unwrap()
    }

    pub fn get_destination(&self) -> String {
        self.configs.destination.clone()
    }

    pub fn get_filter(&self) -> String {
        format!(
            "{}{}",
            self.configs
                .styles
                .iter()
                .map(|s| format!("{}\n", s.get_style(self.configs.palette.clone())))
                .collect::<String>(),
            self.rules
                .iter()
                .map(|f| format!("{}\n", f.get_rule(self.configs.palette.clone())))
                .collect::<String>()
        )
    }
}
