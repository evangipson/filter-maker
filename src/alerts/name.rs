use std::fmt::Display;

/// [`AlertName`] is a collection of names that represent pre-defined alert sounds.
#[derive(PartialEq)]
pub enum AlertName {
    /// [`AlertName::Silent`] will mute any alert.
    Silent,
    /// [`AlertName::Gong`] will play a gong-like sound.
    Gong,
    /// [`AlertName::HollowDrum`] will play a hollow drum sound.
    HollowDrum,
    /// [`AlertName::VoiceDrum`] will play a drum sound layered with a vocal sound.
    VoiceDrum,
    /// [`AlertName::VoiceWhoosh`] will play a whoosh sound layered with a vocal sound.
    VoiceWhoosh,
    /// [`AlertName::LowWhoosh`] will play a low, whooshing sound.
    LowWhoosh,
    /// [`AlertName::Glitter`] will play a fanciful, glittery "schwing" sound.
    Glitter,
    /// [`AlertName::TonalPulse`] will play a medium-register pulsating sound.
    TonalPulse,
    /// [`AlertName::MechanicalPulse`] will play a robotic pulsating sound.
    MechanicalPulse,
    /// [`AlertName::SlowPulse`] will play a slow pulsating sound.
    SlowPulse,
}

/// Implement [`Display`] for [`AlertName`].
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
