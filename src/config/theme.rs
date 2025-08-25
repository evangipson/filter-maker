use serde_derive::Deserialize;

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Theme {
    pub font: String,
    pub background: String,
    pub outline: Option<String>,
}
