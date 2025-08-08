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

pub struct AlertSound {
    pub name: AlertName,
    pub volume: AlertVolume,
}

impl AlertSound {
    pub const LOW_WHOOSH: AlertSound = AlertSound::new(AlertName::LowWhoosh, AlertVolume::Normal);
    pub const LOUD_GLITTER: AlertSound = AlertSound::new(AlertName::Glitter, AlertVolume::Loud);
    pub const NONE: AlertSound = AlertSound::new(AlertName::Silent, AlertVolume::Silent);

    const fn new(name: AlertName, volume: AlertVolume) -> Self {
        Self { name, volume }
    }
}

impl Display for AlertSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name == AlertName::Silent || self.volume == AlertVolume::Silent {
            Ok(())
        } else {
            write!(f, "PlayAlertSound {} {}", self.name, self.volume)
        }
    }
}

pub struct Effect {
    pub alert_sound: AlertSound,
}

impl Effect {
    pub const NONE: Effect = Effect::new(AlertSound::NONE);
    pub const NORMAL_DROP: Effect = Effect::new(AlertSound::LOW_WHOOSH);
    pub const BIG_DROP: Effect = Effect::new(AlertSound::LOUD_GLITTER);

    const fn new(alert_sound: AlertSound) -> Self {
        Self { alert_sound }
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alert_sound)
    }
}
