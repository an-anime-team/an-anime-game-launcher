use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

pub mod fsr;
pub mod hud;
pub mod gamescope;

pub mod prelude {
    pub use super::gamescope::prelude::*;

    pub use super::Enhancements;
    pub use super::fsr::Fsr;
    pub use super::hud::HUD;
}

use prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Enhancements {
    pub fsr: Fsr,
    pub gamemode: bool,
    pub hud: HUD,
    pub gamescope: Gamescope
}

impl From<&JsonValue> for Enhancements {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            fsr: match value.get("fsr") {
                Some(value) => Fsr::from(value),
                None => default.fsr
            },

            gamemode: match value.get("gamemode") {
                Some(value) => value.as_bool().unwrap_or(default.gamemode),
                None => default.gamemode
            },

            hud: match value.get("hud") {
                Some(value) => HUD::from(value),
                None => default.hud
            },

            gamescope: match value.get("gamescope") {
                Some(value) => Gamescope::from(value),
                None => default.gamescope
            }
        }
    }
}
