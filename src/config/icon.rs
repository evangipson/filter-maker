use crate::{behavior::conditional::Conditional, config::common};
use serde_derive::Deserialize;
use std::fmt::Display;

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Icon {
    pub icon_type: String,
    pub color: String,
    pub size: String,
}

impl Icon {
    pub fn get_size_id(&self) -> u8 {
        match self.size.to_lowercase().as_str() {
            "large" => 0,
            "small" => 2,
            _ => 1,
        }
    }
}

impl Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            // if it's a default icon config, don't write anything
            Ok(())
        } else {
            write!(
                f,
                "MinimapIcon {} {} {}",
                self.get_size_id(),
                common::capitalize(&self.color),
                common::capitalize(&self.icon_type)
            )
        }
    }
}
