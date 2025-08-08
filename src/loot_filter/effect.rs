use crate::alerts::{beam::AlertBeam, icon::AlertIcon, sound::AlertSound};
use std::fmt::Display;

#[derive(PartialEq)]
pub struct Effect {
    pub alert_sound: AlertSound,
    pub alert_icon: AlertIcon,
    pub alert_beam: AlertBeam,
}

impl Effect {
    pub const NONE: Effect = Effect::new(AlertSound::NONE, AlertIcon::NONE, AlertBeam::None);
    pub const NORMAL_DROP: Effect = Effect::new(
        AlertSound::LOW_WHOOSH,
        AlertIcon::SMALL_YELLOW_CIRCLE,
        AlertBeam::None,
    );
    pub const BIG_DROP: Effect = Effect::new(
        AlertSound::LOUD_GLITTER,
        AlertIcon::BIG_RED_STAR,
        AlertBeam::Red,
    );
    pub const GOLD_PILE: Effect = Effect::new(
        AlertSound::QUIET_GONG,
        AlertIcon::YELLOW_STAR,
        AlertBeam::None,
    );

    const fn new(alert_sound: AlertSound, alert_icon: AlertIcon, alert_beam: AlertBeam) -> Self {
        Self {
            alert_sound,
            alert_icon,
            alert_beam,
        }
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alert_sound = format!("{}", self.alert_sound);
        let alert_icon = format!("{}", self.alert_icon);
        let alert_beam = format!("{}", self.alert_beam);
        write!(
            f,
            "{}",
            [alert_sound, alert_icon, alert_beam]
                .into_iter()
                .filter(|line| !line.is_empty())
                .map(|line| line + "\n")
                .collect::<String>()
                .strip_suffix("\n")
                .unwrap_or_default(),
        )
    }
}

impl Default for Effect {
    fn default() -> Self {
        Self {
            alert_sound: AlertSound::NONE,
            alert_icon: AlertIcon::NONE,
            alert_beam: AlertBeam::None,
        }
    }
}
