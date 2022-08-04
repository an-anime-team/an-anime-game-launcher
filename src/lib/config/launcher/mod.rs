use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::consts::launcher_dir;

pub mod repairer;

pub mod prelude {
    pub use super::Launcher;
    pub use super::repairer::Repairer;
}

use prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    pub language: String,
    pub temp: Option<String>,
    pub repairer: Repairer
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            language: String::from("en-us"),
            temp: launcher_dir(),
            repairer: Repairer::default()
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
            }
        }
    }
}
