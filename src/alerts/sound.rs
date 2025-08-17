use crate::alerts::{name::AlertName, volume::AlertVolume};
use std::fmt::Display;

/// [`AlertSound`] represents a sound for an alert.
#[derive(PartialEq)]
pub struct AlertSound {
    /// [`AlertSound::name`] represents the type of sound for an alert.
    pub name: AlertName,
    /// [`AlertSound::volume`] represents the loudness of an alert sound.
    pub volume: AlertVolume,
}

/// Implement [`AlertSound`].
impl AlertSound {
    /// [`AlertSound::LOW_WHOOSH`] will play a low whooshing alert sound at a normal volume.
    pub const LOW_WHOOSH: AlertSound = AlertSound::new(AlertName::LowWhoosh, AlertVolume::Normal);

    /// [`AlertSound::HOLLOW_DRUM`] will play a hollow drum alert sound at a normal volume.
    pub const HOLLOW_DRUM: AlertSound = AlertSound::new(AlertName::HollowDrum, AlertVolume::Normal);

    /// [`AlertSound::LOUD_GLITTER`] will play a "schwing" alert sound at a loud volume.
    pub const LOUD_GLITTER: AlertSound = AlertSound::new(AlertName::Glitter, AlertVolume::Loud);

    /// [`AlertSound::QUIET_GONG`] will play a gong alert sound quietly.
    pub const QUIET_GONG: AlertSound = AlertSound::new(AlertName::Gong, AlertVolume::Quiet);

    /// [`AlertSound::QUIET_DRUM`] will play a vocal drum alert sound quietly.
    pub const QUIET_DRUM: AlertSound = AlertSound::new(AlertName::VoiceDrum, AlertVolume::Quiet);

    /// [`AlertSound::LOUD_PULSE`] will play a tonal pulse alert sound at a loud volume.
    pub const LOUD_PULSE: AlertSound = AlertSound::new(AlertName::TonalPulse, AlertVolume::Loud);

    /// [`AlertSound::NONE`] will not play any alert sound.
    pub const NONE: AlertSound = AlertSound::new(AlertName::Silent, AlertVolume::Silent);

    /// [`AlertSound::new`] will create a new [`AlertSound`].
    const fn new(name: AlertName, volume: AlertVolume) -> Self {
        Self { name, volume }
    }
}

/// Implement [`Display`] for [`AlertSound`].
impl Display for AlertSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name == AlertName::Silent || self.volume == AlertVolume::Silent {
            Ok(())
        } else {
            write!(f, "PlayAlertSound {} {}", self.name, self.volume)
        }
    }
}
