use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::consts::launcher_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dxvk {
    pub builds: PathBuf
}

impl Default for Dxvk {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            builds: launcher_dir.join("dxvks")
        }
    }
}

impl From<&JsonValue> for Dxvk {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            builds: match value.get("builds") {
                Some(value) => match value.as_str() {
                    Some(value) => PathBuf::from(value),
                    None => default.builds
                },
                None => default.builds
            }
        }
    }
}
