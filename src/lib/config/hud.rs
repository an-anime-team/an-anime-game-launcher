use std::collections::HashMap;

use serde::{Serialize, Deserialize};

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
    pub fn get_env_vars(&self) -> HashMap<&str, &str> {
        match self {
            Self::None => HashMap::new(),
            Self::DXVK => HashMap::from([
                ("DXVK_HUD", "1")
            ]),
            Self::MangoHUD => HashMap::from([
                ("MANGOHUD", "1")
            ])
        }
    }
}
