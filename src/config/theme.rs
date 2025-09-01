use serde_derive::Deserialize;

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Theme {
    pub font: Option<String>,
    pub background: Option<String>,
    pub outline: Option<String>,
}

impl Theme {
    pub fn new(
        font: &Option<String>,
        background: &Option<String>,
        outline: &Option<String>,
    ) -> Option<Self> {
        Some(Self {
            font: font.clone(),
            background: background.clone(),
            outline: outline.clone(),
        })
    }
}
