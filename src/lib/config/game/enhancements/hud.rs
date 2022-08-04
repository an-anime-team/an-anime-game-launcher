use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::config::Config;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HUD {
    None,
    DXVK,
    MangoHUD
}

impl Default for HUD {
    fn default() -> Self {
        Self::None
    }
}

impl From<&JsonValue> for HUD {
    fn from(value: &JsonValue) -> Self {
        serde_json::from_value(value.clone()).unwrap_or(Self::default())
    }
}

impl TryFrom<u32> for HUD {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::DXVK),
            2 => Ok(Self::MangoHUD),
            _ => Err(String::from("Failed to convert number to HUD enum"))
        }
    }
}

impl Into<u32> for HUD {
    fn into(self) -> u32 {
        match self {
            Self::None     => 0,
            Self::DXVK     => 1,
            Self::MangoHUD => 2
        }
    }
}

impl HUD {
    /// Get environment variables corresponding to used wine hud
    pub fn get_env_vars(&self, config: &Config) -> HashMap<&str, &str> {
        match self {
            Self::None => HashMap::new(),
            Self::DXVK => HashMap::from([
                ("DXVK_HUD", "fps,frametimes,version,gpuload")
            ]),
            Self::MangoHUD => {
                // Don't show mangohud if gamescope is enabled
                // otherwise it'll be doubled
                if config.game.enhancements.gamescope.enabled {
                    HashMap::new()
                } else {
                    HashMap::from([
                        ("MANGOHUD", "1")
                    ])
                }
            }
        }
    }
}
