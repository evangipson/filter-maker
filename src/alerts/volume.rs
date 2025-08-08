use std::fmt::Display;

#[derive(PartialEq)]
pub enum AlertVolume {
    Silent,
    Quiet,
    Normal,
    Loud,
}

impl Display for AlertVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Silent => "",
                Self::Quiet => "100",
                Self::Normal => "200",
                Self::Loud => "300",
            }
        )
    }
}
