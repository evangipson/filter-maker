use std::fmt::Display;

/// [`Rarity`] represents different rarities of an item.
#[derive(Default, PartialEq)]
pub enum Rarity {
    /// [`Rarity::None`] represents an item with no rarity, and is the default value.
    #[default]
    None,
    /// [`Rarity::Normal`] represents an item with normal rarity ("white" item).
    Normal,
    /// [`Rarity::Magic`] represents an item with magic rarity ("blue" item).
    Magic,
    /// [`Rarity::Rare`] represents a rare item ("yellow" item).
    Rare,
    /// [`Rarity::Unique`] represents a unique item ("orange" item).
    Unique,
    /// [`Rarity::All`] represents all item rarities.
    All,
}

/// Implement [`Display`] for [`Rarity`].
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
