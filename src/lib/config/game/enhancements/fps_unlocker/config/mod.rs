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
    pub monitor: u64,
    pub window_mode: u64,
    pub priority: u64
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fps: 120,
            power_saving: false,
            monitor: 1,
            window_mode: 0,
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

            monitor: match value.get("monitor") {
                Some(value) => value.as_u64().unwrap_or(default.monitor),
                None => default.monitor
            },

            window_mode: match value.get("window_mode") {
                Some(value) => value.as_u64().unwrap_or(default.window_mode),
                None => default.window_mode
            },

            priority: match value.get("priority") {
                Some(value) => value.as_u64().unwrap_or(default.priority),
                None => default.priority
            }
        }
    }
}
