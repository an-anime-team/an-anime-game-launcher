use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

pub mod fps;

pub mod prelude {
    pub use super::fps::Fps;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub fps: u64,
    pub power_saving: bool,
    pub fullscreen: bool,
    pub priority: u64
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fps: 120,
            power_saving: false,
            fullscreen: false,
            priority: 3
        }
    }
}

impl From<&JsonValue> for Config {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            fps: match value.get("fps") {
                Some(value) => value.as_u64().unwrap_or(default.fps),
                None => default.fps
            },

            power_saving: match value.get("power_saving") {
                Some(value) => value.as_bool().unwrap_or(default.power_saving),
                None => default.power_saving
            },

            fullscreen: match value.get("fullscreen") {
                Some(value) => value.as_bool().unwrap_or(default.fullscreen),
                None => default.fullscreen
            },

            priority: match value.get("priority") {
                Some(value) => value.as_u64().unwrap_or(default.priority),
                None => default.priority
            }
        }
    }
}
