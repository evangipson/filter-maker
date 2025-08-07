use std::fmt::Display;

#[derive(PartialEq)]
pub enum Rarity {
    All,
    Common,
    Magic,
    Rare,
    Unique,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => "",
                Self::Common => "Common",
                Self::Magic => "Magic",
                Self::Rare => "Rare",
                Self::Unique => "Unique",
            }
        )
    }
}
