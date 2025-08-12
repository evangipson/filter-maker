use crate::alerts::color::Color;
use std::fmt::Display;

/// [`AlertBeam`] represents a beam coming from an item on the map.
#[derive(PartialEq)]
pub struct AlertBeam {
    /// [`AlertBeam::color`] represents the color of an [`AlertBeam`].
    pub color: Color,
}

/// Implement [`AlertBeam`].
impl AlertBeam {
    /// [`AlertBeam::NONE`] will not display any beam for an item.
    pub const NONE: AlertBeam = AlertBeam::new(Color::None);

    /// [`AlertBeam::RED`] will display a red beam coming from an item.
    pub const RED: AlertBeam = AlertBeam::new(Color::Red);

    /// [`AlertBeam::GREEN`] will display a green beam coming from an item.
    pub const GREEN: AlertBeam = AlertBeam::new(Color::Green);

    /// [`AlertBeam::BLUE`] will display a blue beam coming from an item.
    pub const BLUE: AlertBeam = AlertBeam::new(Color::Blue);

    /// [`AlertBeam::BROWN`] will display a brown beam coming from an item.
    pub const BROWN: AlertBeam = AlertBeam::new(Color::Brown);

    /// [`AlertBeam::WHITE`] will display a white beam coming from an item.
    pub const WHITE: AlertBeam = AlertBeam::new(Color::White);

    /// [`AlertBeam::YELLOW`] will display a yellow beam coming from an item.
    pub const YELLOW: AlertBeam = AlertBeam::new(Color::Yellow);

    /// [`AlertBeam::CYAN`] will display a cyan beam coming from an item.
    pub const CYAN: AlertBeam = AlertBeam::new(Color::Cyan);

    /// [`AlertBeam::GREY`] will display a grey beam coming from an item.
    pub const GREY: AlertBeam = AlertBeam::new(Color::Grey);

    /// [`AlertBeam::ORANGE`] will display an orange beam coming from an item.
    pub const ORANGE: AlertBeam = AlertBeam::new(Color::Orange);

    /// [`AlertBeam::PINK`] will display a pink beam coming from an item.
    pub const PINK: AlertBeam = AlertBeam::new(Color::Pink);

    /// [`AlertBeam::PURPLE`] will display a purple beam coming from an item.
    pub const PURPLE: AlertBeam = AlertBeam::new(Color::Purple);

    /// [`AlertBeam::new`] will create a new [`AlertBeam`] using a [`Color`].
    const fn new(color: Color) -> Self {
        Self { color }
    }
}

/// Implement [`Display`] for [`AlertBeam`].
impl Display for AlertBeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.color {
                Color::None => "",
                Color::Red => "PlayEffect Red",
                Color::Green => "PlayEffect Green",
                Color::Blue => "PlayEffect Blue",
                Color::Brown => "PlayEffect Brown",
                Color::White => "PlayEffect White",
                Color::Yellow => "PlayEffect Yellow",
                Color::Cyan => "PlayEffect Cyan",
                Color::Grey => "PlayEffect Grey",
                Color::Orange => "PlayEffect Orange",
                Color::Pink => "PlayEffect Pink",
                Color::Purple => "PlayEffect Purple",
            }
        )
    }
}
