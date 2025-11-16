use serde_derive::Deserialize;

/// [`Modifier`] is a rust representation of an item modifier defined in the
/// TOML configuration file.
#[derive(Default, Deserialize, PartialEq)]
pub struct Modifier {
    /// [`Modifier::classes`] is an optional collection of item classes.
    pub classes: Option<Vec<String>>,
    /// [`Modifier::good_mods`] is an optional collection of good item
    /// modifiers.
    pub good_mods: Option<Vec<String>>,
}
