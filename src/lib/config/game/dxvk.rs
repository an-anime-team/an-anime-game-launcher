use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::consts::launcher_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dxvk {
    pub builds: String
}

impl Default for Dxvk {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            builds: format!("{launcher_dir}/dxvks")
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
            }
        }
    }
}
