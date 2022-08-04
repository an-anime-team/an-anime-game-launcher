use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Fsr {
    pub strength: u64,
    pub enabled: bool
}

impl Default for Fsr {
    fn default() -> Self {
        Self {
            strength: 2,
            enabled: false
        }
    }
}

impl From<&JsonValue> for Fsr {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            strength: match value.get("strength") {
                Some(value) => value.as_u64().unwrap_or(default.strength),
                None => default.strength
            },

            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            }
        }
    }
}

impl Fsr {
    /// Get environment variables corresponding to used amd fsr options
    pub fn get_env_vars(&self) -> HashMap<&str, String> {
        if self.enabled {
            HashMap::from([
                ("WINE_FULLSCREEN_FSR", String::from("1")),
                ("WINE_FULLSCREEN_FSR_STRENGTH", self.strength.to_string())
            ])
        }
        
        else {
            HashMap::new()
        }
    }
}
