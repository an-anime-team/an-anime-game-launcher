use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::consts::launcher_dir;

pub mod wine_sync;
pub mod wine_lang;

pub mod prelude {
    pub use super::Wine;
    pub use super::wine_sync::WineSync;
    pub use super::wine_lang::WineLang;
}

use prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wine {
    pub prefix: String,
    pub builds: String,
    pub selected: Option<String>,
    pub sync: WineSync,
    pub language: WineLang
}

impl Default for Wine {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            prefix: format!("{launcher_dir}/game"),
            builds: format!("{launcher_dir}/runners"),
            selected: None,
            sync: WineSync::default(),
            language: WineLang::default()
        }
    }
}

impl From<&JsonValue> for Wine {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            prefix: match value.get("prefix") {
                Some(value) => value.as_str().unwrap_or(&default.prefix).to_string(),
                None => default.prefix
            },

            builds: match value.get("builds") {
                Some(value) => value.as_str().unwrap_or(&default.builds).to_string(),
                None => default.builds
            },

            selected: match value.get("selected") {
                Some(value) => {
                    if value.is_null() {
                        None
                    } else {
                        match value.as_str() {
                            Some(value) => Some(value.to_string()),
                            None => default.selected
                        }
                    }
                },
                None => default.selected
            },

            sync: match value.get("sync") {
                Some(value) => WineSync::from(value),
                None => default.sync
            },

            language: match value.get("language") {
                Some(value) => WineLang::from(value),
                None => default.language
            }
        }
    }
}
