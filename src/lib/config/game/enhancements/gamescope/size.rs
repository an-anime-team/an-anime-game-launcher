use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Size {
    pub width: u64,
    pub height: u64
}

impl From<&JsonValue> for Size {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            width: match value.get("width") {
                Some(value) => value.as_u64().unwrap_or(default.width),
                None => default.width
            },

            height: match value.get("height") {
                Some(value) => value.as_u64().unwrap_or(default.height),
                None => default.height
            }
        }
    }
}
