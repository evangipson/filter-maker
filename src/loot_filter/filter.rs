use std::fmt::Display;

use crate::loot_filter::rule::Rule;

#[derive(Default)]
pub struct Filter {
    pub rules: Vec<Rule>,
}

impl Filter {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.rules
                .iter()
                .map(|rule| rule.to_string())
                .collect::<String>()
        )
    }
}
