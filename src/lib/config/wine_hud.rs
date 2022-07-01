use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WineHUD {
    None,
    DXVK,
    MangoHUD
}

impl Default for WineHUD {
    fn default() -> Self {
        Self::None
    }
}

impl TryFrom<u32> for WineHUD {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::DXVK),
            2 => Ok(Self::MangoHUD),
            _ => Err(String::from("Failed to convert number to WineHUD enum"))
        }
    }
}

impl Into<u32> for WineHUD {
    fn into(self) -> u32 {
        match self {
            Self::None => 0,
            Self::DXVK => 1,
            Self::MangoHUD => 2
        }
    }
}
