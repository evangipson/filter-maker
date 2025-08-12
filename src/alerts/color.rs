use std::fmt::Display;

/// [`Color`] represents a collection of pre-defined colors.
#[derive(PartialEq)]
pub enum Color {
    /// [`Color::None`] represents no color.
    None,
    /// [`Color::Red`] represents a pre-defined red color.
    Red,
    /// [`Color::Green`] represents a pre-defined green color.
    Green,
    /// [`Color::Blue`] represents a pre-defined blue color.
    Blue,
    /// [`Color::Brown`] represents a pre-defined brown color.
    Brown,
    /// [`Color::White`] represents a pre-defined white color.
    White,
    /// [`Color::Yellow`] represents a pre-defined yellow color.
    Yellow,
    /// [`Color::Cyan`] represents a pre-defined cyan color.
    Cyan,
    /// [`Color::Grey`] represents a pre-defined grey color.
    Grey,
    /// [`Color::Orange`] represents a pre-defined orange color.
    Orange,
    /// [`Color::Pink`] represents a pre-defined pink color.
    Pink,
    /// [`Color::Purple`] represents a pre-defined purple color.
    Purple,
}

/// Implement [`Display`] for [`Color`].
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::None => "",
                Color::Red => "Red",
                Color::Green => "Green",
                Color::Blue => "Blue",
                Color::Brown => "Brown",
                Color::White => "White",
                Color::Yellow => "Yellow",
                Color::Cyan => "Cyan",
                Color::Grey => "Grey",
                Color::Orange => "Orange",
                Color::Pink => "Pink",
                Color::Purple => "Purple",
            }
        )
    }
}
