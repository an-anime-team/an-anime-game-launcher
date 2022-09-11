use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

pub mod fps;

pub mod prelude {
    pub use super::fps::Fps;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub fps: u64,
    pub power_saving: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fps: 120,
            power_saving: false
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
            }
        }
    }
}
