use crate::behavior::{common, conditional::Conditional};
use serde_derive::Deserialize;
use std::fmt::Display;

/// [`Icon`] is a rust representation of an icon defined in the TOML
/// configuration file.
#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Icon {
    /// [`Icon::icon_type`] is an identifier for a type of [`Icon`].
    pub icon_type: String,
    /// [`Icon::color`] is the color of an [`Icon`].
    pub color: String,
    /// [`Icon::size`] is the size of an [`Icon`].
    pub size: String,
}

impl Icon {
    /// [`Icon::get_size_id`] will transform [`Icon::size`] into a
    /// number that is understood by Path of Exile's item filters.
    /// # Example
    /// [`Icon::get_size_id`] can be used to get the correct number
    /// to represent an icon size:
    /// ```rust
    /// use filter_maker::config::icon::Icon;
    ///
    /// fn get_icon_size_number(icon: &Icon) -> u8 {
    ///     icon.get_size_id()
    /// }
    /// ```
    pub fn get_size_id(&self) -> u8 {
        match self.size.to_lowercase().as_str() {
            "large" => 0,
            "small" => 2,
            _ => 1,
        }
    }
}

/// Implement [`Display`] for [`Icon`].
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
