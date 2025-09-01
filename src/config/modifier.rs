use serde_derive::Deserialize;

#[derive(Default, Deserialize, PartialEq)]
pub struct Modifier {
    pub classes: Option<Vec<String>>,
    pub good_mods: Option<Vec<String>>,
}
