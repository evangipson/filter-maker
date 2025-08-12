use crate::alerts::{beam::AlertBeam, icon::AlertIcon, sound::AlertSound};
use std::fmt::Display;

/// [`Effect`] represents a sound, minimap icon, or beam effect for an item.
#[derive(PartialEq)]
pub struct Effect {
    /// [`Effect::alert_sound`] is an [`AlertSound`] that plays when an item is dropped.
    pub alert_sound: AlertSound,
    /// [`Effect::alert_icon`] is an [`AlertIcon`] shown on the minimap when an item is dropped.
    pub alert_icon: AlertIcon,
    /// [`Effect::alert_beam`] is an [`AlertBeam`] that is shown on an item when it drops.
    pub alert_beam: AlertBeam,
}

/// Implement [`Effect`].
impl Effect {
    /// [`Effect::NONE`] represents no sound, minimap icon, or beam effect when an item is dropped.
    pub const NONE: Effect = Effect::new(AlertSound::NONE, AlertIcon::NONE, AlertBeam::NONE);

    /// [`Effect::NORMAL_DROP`] will play a low whooshing sound and show a small yellow circle minimap
    /// icon when an item is dropped.
    pub const NORMAL_DROP: Effect = Effect::new(
        AlertSound::LOW_WHOOSH,
        AlertIcon::SMALL_YELLOW_CIRCLE,
        AlertBeam::NONE,
    );

    /// [`Effect::INTERESTING_DROP`] will play a hollow drum sound, show a purple kite minimap icon and
    /// a purple beam when an item is dropped.
    pub const INTERESTING_DROP: Effect = Effect::new(
        AlertSound::HOLLOW_DRUM,
        AlertIcon::PURPLE_KITE,
        AlertBeam::PURPLE,
    );

    /// [`Effect::LINKED_DROP`] will play a low whooshing sound, show a small red circle minimap icon and
    /// a red beam when an item is dropped.
    pub const LINKED_DROP: Effect = Effect::new(
        AlertSound::LOW_WHOOSH,
        AlertIcon::SMALL_RED_CIRCLE,
        AlertBeam::RED,
    );

    /// [`Effect::BIG_DROP`] will play a loud "schwing" sound, show a big red star minimap icon and a red
    /// beam when an item is dropped.
    pub const BIG_DROP: Effect = Effect::new(
        AlertSound::LOUD_GLITTER,
        AlertIcon::BIG_RED_STAR,
        AlertBeam::RED,
    );

    /// [`Effect::GOLD_PILE`] will play a quiet gong sound, show a yellow star minimap icon and yellow beam
    /// when an item is dropped.
    pub const GOLD_PILE: Effect = Effect::new(
        AlertSound::QUIET_GONG,
        AlertIcon::YELLOW_STAR,
        AlertBeam::YELLOW,
    );

    /// [`Effect::SMALL_DROP`] will play a quiet drum sound and show a small white circle minimap icon when
    /// an item is dropped.
    pub const SMALL_DROP: Effect = Effect::new(
        AlertSound::QUIET_DRUM,
        AlertIcon::SMALL_WHITE_CIRCLE,
        AlertBeam::NONE,
    );

    /// [`Effect::new`] will create a new [`Effect`].
    const fn new(alert_sound: AlertSound, alert_icon: AlertIcon, alert_beam: AlertBeam) -> Self {
        Self {
            alert_sound,
            alert_icon,
            alert_beam,
        }
    }
}

/// Implement [`Display`] for [`Effect`].
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

/// Implement [`Default`] for [`Effect`].
impl Default for Effect {
    fn default() -> Self {
        Self {
            alert_sound: AlertSound::NONE,
            alert_icon: AlertIcon::NONE,
            alert_beam: AlertBeam::NONE,
        }
    }
}
