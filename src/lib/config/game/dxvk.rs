use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::consts::launcher_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dxvk {
    pub builds: String,
    pub selected: Option<String>
}

impl Default for Dxvk {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            builds: format!("{launcher_dir}/dxvks"),
            selected: None
        }
    }
}

impl From<&JsonValue> for Dxvk {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
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
            }
        }
    }
}
