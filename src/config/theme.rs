use serde_derive::Deserialize;

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Theme {
    pub font: String,
    pub background: String,
    pub outline: Option<String>,
}

impl Theme {
    pub fn new(
        font: &Option<String>,
        background: &Option<String>,
        outline: &Option<String>,
    ) -> Option<Self> {
        Some(Self {
            font: font.clone().unwrap_or_default(),
            background: background.clone().unwrap_or_default(),
            outline: outline.clone(),
        })
    }
}
