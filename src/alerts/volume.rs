use std::fmt::Display;

/// [`AlertVolume`] is a collection of pre-defined values to control the volume of alerts.
#[derive(PartialEq)]
pub enum AlertVolume {
    /// [`AlertVolume::Silent`] will mute any alert sound.
    Silent,
    /// [`AlertVolume::Quiet`] will play an alert sound at a quiet volume.
    Quiet,
    /// [`AlertVolume::Normal`] will play an alert sound at a normal volume.
    Normal,
    /// [`AlertVolume::Loud`] will play an alert sound at a loud volume.
    Loud,
}

/// Implement [`Display`] for [`AlertVolume`].
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
