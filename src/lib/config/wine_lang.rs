use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WineLang {
    System,
    English,
    German,
    Russian,
    Portuguese,
    French,
    Chinese,
    Spanish,
    Japanese,
    Korean
}

impl Default for WineLang {
    fn default() -> Self {
        Self::System
    }
}

impl TryFrom<u32> for WineLang {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::System),
            1 => Ok(Self::English),
            2 => Ok(Self::German),
            3 => Ok(Self::Russian),
            4 => Ok(Self::Portuguese),
            5 => Ok(Self::French),
            6 => Ok(Self::Chinese),
            7 => Ok(Self::Spanish),
            8 => Ok(Self::Japanese),
            9 => Ok(Self::Korean),
            _ => Err(String::from("Failed to convert number to WineLang enum"))
        }
    }
}

impl Into<u32> for WineLang {
    fn into(self) -> u32 {
        match self {
            WineLang::System     => 0,
            WineLang::English    => 1,
            WineLang::German     => 2,
            WineLang::Russian    => 3,
            WineLang::Portuguese => 4,
            WineLang::French     => 5,
            WineLang::Chinese    => 6,
            WineLang::Spanish    => 7,
            WineLang::Japanese   => 8,
            WineLang::Korean     => 9
        }
    }
}

impl WineLang {
    /// Get environment variables corresponding to used wine language
    pub fn get_env_vars(&self) -> HashMap<&str, &str> {
        HashMap::from([("LANG", match self {
            WineLang::System => return HashMap::new(),

            WineLang::English    => "en_US.UTF8",
            WineLang::German     => "de_DE.UTF8",
            WineLang::Russian    => "ru_RU.UTF8",
            WineLang::Portuguese => "pt_PT.UTF8",
            WineLang::French     => "fr_FR.UTF8",
            WineLang::Chinese    => "zh_CN.UTF8",
            WineLang::Spanish    => "es_ES.UTF8",
            WineLang::Japanese   => "ja_JP.UTF8",
            WineLang::Korean     => "ko_KR.UTF8"
        })])
    }
}
