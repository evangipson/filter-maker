use std::fmt::Display;

#[derive(Default, PartialEq)]
pub enum Rarity {
    #[default]
    None,
    Normal,
    Magic,
    Rare,
    Unique,
    All,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Normal => "Normal",
                Self::Magic => "Magic",
                Self::Rare => "Rare",
                Self::Unique => "Unique",
                Self::All => "Normal Magic Rare Unique",
            }
        )
    }
}
