use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use anime_game_core::genshin::consts::GameEdition as CoreGameEdition;

use crate::lib::consts::launcher_dir;

pub mod repairer;

pub mod prelude {
    pub use super::Launcher;
    pub use super::repairer::Repairer;
}

use prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEdition {
    Global,
    China
}

impl Default for GameEdition {
    fn default() -> Self {
        let locale = match std::env::var("LC_ALL") {
            Ok(locale) => locale,
            Err(_) => match std::env::var("LC_MESSAGES") {
                Ok(locale) => locale,
                Err(_) => match std::env::var("LANG") {
                    Ok(locale) => locale,
                    Err(_) => return Self::Global
                }
            }
        };

        if locale.len() > 4 && &locale[..5].to_lowercase() == "zh_cn" {
            Self::China
        } else {
            Self::Global
        }
    }
}

impl Into<CoreGameEdition> for GameEdition {
    fn into(self) -> CoreGameEdition {
        match self {
            Self::Global => CoreGameEdition::Global,
            Self::China  => CoreGameEdition::China
        }
    }
}

impl From<CoreGameEdition> for GameEdition {
    fn from(edition: CoreGameEdition) -> Self {
        match edition {
            CoreGameEdition::Global => Self::Global,
            CoreGameEdition::China  => Self::China
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    pub language: String,
    pub temp: Option<String>,
    pub repairer: Repairer,
    pub edition: GameEdition
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            language: String::from("en-us"),
            temp: launcher_dir(),
            repairer: Repairer::default(),
            edition: GameEdition::default()
        }
    }
}

impl From<&JsonValue> for Launcher {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            language: match value.get("language") {
                Some(value) => value.as_str().unwrap_or(&default.language).to_string(),
                None => default.language
            },

            temp: match value.get("temp") {
                Some(value) => {
                    if value.is_null() {
                        None
                    } else {
                        match value.as_str() {
                            Some(value) => Some(value.to_string()),
                            None => default.temp
                        }
                    }
                },
                None => default.temp
            },

            repairer: match value.get("repairer") {
                Some(value) => Repairer::from(value),
                None => default.repairer
            },

            edition: match value.get("edition") {
                Some(value) => serde_json::from_value(value.clone()).unwrap_or(default.edition),
                None => default.edition
            }
        }
    }
}
