use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WineSync {
    None,
    ESync,
    FSync,
    Futex2
}

impl Default for WineSync {
    fn default() -> Self {
        Self::FSync
    }
}

impl From<&JsonValue> for WineSync {
    fn from(value: &JsonValue) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}

impl TryFrom<u32> for WineSync {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::ESync),
            2 => Ok(Self::FSync),
            3 => Ok(Self::Futex2),

            _ => Err(String::from("Failed to convert number to WineSync enum"))
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for WineSync {
    fn into(self) -> u32 {
        match self {
            Self::None   => 0,
            Self::ESync  => 1,
            Self::FSync  => 2,
            Self::Futex2 => 3
        }
    }
}

impl WineSync {
    /// Get environment variables corresponding to used wine sync
    pub fn get_env_vars(&self) -> HashMap<&str, &str> {
        HashMap::from([(match self {
            Self::None => return HashMap::new(),

            Self::ESync  => "WINEESYNC",
            Self::FSync  => "WINEFSYNC",
            Self::Futex2 => "WINEFSYNC_FUTEX2"
        }, "1")])
    }
}
