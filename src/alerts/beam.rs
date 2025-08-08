use std::fmt::Display;

#[derive(PartialEq)]
pub enum AlertBeam {
    None,
    Red,
    Green,
    Blue,
    Brown,
    White,
    Yellow,
    Cyan,
    Grey,
    Orange,
    Pink,
    Purple,
}

impl Display for AlertBeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AlertBeam::None => "",
                AlertBeam::Red => "PlayEffect Red",
                AlertBeam::Green => "PlayEffect Green",
                AlertBeam::Blue => "PlayEffect Blue",
                AlertBeam::Brown => "PlayEffect Brown",
                AlertBeam::White => "PlayEffect White",
                AlertBeam::Yellow => "PlayEffect Yellow",
                AlertBeam::Cyan => "PlayEffect Cyan",
                AlertBeam::Grey => "PlayEffect Grey",
                AlertBeam::Orange => "PlayEffect Orange",
                AlertBeam::Pink => "PlayEffect Pink",
                AlertBeam::Purple => "PlayEffect Purple",
            }
        )
    }
}
