use crate::alerts::{name::AlertName, volume::AlertVolume};
use std::fmt::Display;

#[derive(PartialEq)]
pub struct AlertSound {
    pub name: AlertName,
    pub volume: AlertVolume,
}

impl AlertSound {
    pub const LOW_WHOOSH: AlertSound = AlertSound::new(AlertName::LowWhoosh, AlertVolume::Normal);
    pub const HOLLOW_DRUM: AlertSound = AlertSound::new(AlertName::HollowDrum, AlertVolume::Normal);
    pub const LOUD_GLITTER: AlertSound = AlertSound::new(AlertName::Glitter, AlertVolume::Loud);
    pub const QUIET_GONG: AlertSound = AlertSound::new(AlertName::Gong, AlertVolume::Quiet);
    pub const QUIET_DRUM: AlertSound = AlertSound::new(AlertName::VoiceDrum, AlertVolume::Quiet);
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
