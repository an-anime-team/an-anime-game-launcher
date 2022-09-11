use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::consts::launcher_dir;

pub mod config;

pub mod prelude {
    pub use super::config::Config;

    pub use super::config::prelude::*;
}

use prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpsUnlocker {
    pub path: String,
    pub enabled: bool,
    pub config: Config
}

impl Default for FpsUnlocker {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            path: format!("{launcher_dir}/fps-unlocker"),
            enabled: false,
            config: Config::default()
        }
    }
}

impl From<&JsonValue> for FpsUnlocker {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            path: match value.get("path") {
                Some(value) => value.as_str().unwrap_or(&default.path).to_string(),
                None => default.path
            },

            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            },

            config: match value.get("config") {
                Some(value) => Config::from(value),
                None => default.config
            }
        }
    }
}
