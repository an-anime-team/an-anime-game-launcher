use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repairer {
    pub threads: u64,
    pub fast: bool
}

impl Default for Repairer {
    fn default() -> Self {
        Self {
            threads: 4,
            fast: false
        }
    }
}

impl From<&JsonValue> for Repairer {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            threads: match value.get("threads") {
                Some(value) => value.as_u64().unwrap_or(default.threads),
                None => default.threads
            },

            fast: match value.get("fast") {
                Some(value) => value.as_bool().unwrap_or(default.fast),
                None => default.fast
            }
        }
    }
}
