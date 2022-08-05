use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::config::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VirtualDesktop {
    pub enabled: bool,
    pub width: u64,
    pub height: u64
}

impl Default for VirtualDesktop {
    fn default() -> Self {
        Self {
            enabled: false,
            width: 1920,
            height: 1080
        }
    }
}

impl From<&JsonValue> for VirtualDesktop {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            },

            width: match value.get("width") {
                Some(value) => value.as_u64().unwrap_or(default.width),
                None => default.width
            },

            height: match value.get("height") {
                Some(value) => value.as_u64().unwrap_or(default.height),
                None => default.height
            }
        }
    }
}

impl VirtualDesktop {
    pub fn get_resolution(&self) -> Resolution {
        Resolution::from_pair(self.width, self.height)
    }

    pub fn get_command(&self) -> Option<String> {
        if self.enabled {
            Some(format!("explorer /desktop=animegame,{}x{}", self.width, self.height))
        }

        else {
            None
        }
    }
}
