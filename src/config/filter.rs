use crate::config::{base_config::BaseConfig, rule::Rule};
use serde_derive::Deserialize;
use std::{fs, process::exit};

#[derive(Default, Deserialize, PartialEq)]
pub struct Filter {
    pub configs: BaseConfig,
    pub filters: Vec<Rule>,
}

impl Filter {
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

    pub fn get_destination(&self) -> String {
        self.configs.destination.clone()
    }

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
