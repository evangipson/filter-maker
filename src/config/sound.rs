use crate::behavior::conditional::Conditional;
use serde_derive::Deserialize;
use std::fmt::Display;

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Sound {
    pub sound_type: String,
    pub volume: String,
}

impl Sound {
    pub fn get_sound_id(&self) -> u8 {
        match self.sound_type.to_lowercase().as_str() {
            "gong" => 1,
            "hollow_drum" => 2,
            "voice_drum" => 3,
            "voice_whoosh" => 4,
            "low_whoosh" => 5,
            "glitter" => 6,
            "tonal_pulse" => 7,
            "mechanical_pulse" => 8,
            "slow_pulse" => 9,
            _ => 0,
        }
    }

    pub fn get_volume_id(&self) -> u16 {
        match self.volume.to_lowercase().as_str() {
            "quiet" => 100,
            "normal" => 200,
            "loud" => 300,
            _ => 0,
        }
    }
}

impl Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            // if it's a default sound config, don't write anything
            Ok(())
        } else {
            write!(
                f,
                "PlayAlertSound {} {}",
                self.get_sound_id(),
                self.get_volume_id()
            )
        }
    }
}
