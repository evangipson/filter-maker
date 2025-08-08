use std::fmt::Display;

#[derive(PartialEq)]
pub enum AlertName {
    Silent,
    Gong,
    HollowDrum,
    VoiceDrum,
    VoiceWhoosh,
    LowWhoosh,
    Glitter,
    TonalPulse,
    MechanicalPulse,
    SlowPulse,
}

impl Display for AlertName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Silent => "",
                Self::Gong => "1",
                Self::HollowDrum => "2",
                Self::VoiceDrum => "3",
                Self::VoiceWhoosh => "4",
                Self::LowWhoosh => "5",
                Self::Glitter => "6",
                Self::TonalPulse => "7",
                Self::MechanicalPulse => "8",
                Self::SlowPulse => "9",
            }
        )
    }
}
