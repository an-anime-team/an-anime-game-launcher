use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Framerate {
    pub focused: u64,
    pub unfocused: u64
}

impl From<&JsonValue> for Framerate {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            focused: match value.get("focused") {
                Some(value) => value.as_u64().unwrap_or(default.focused),
                None => default.focused
            },

            unfocused: match value.get("unfocused") {
                Some(value) => value.as_u64().unwrap_or(default.unfocused),
                None => default.unfocused
            }
        }
    }
}
