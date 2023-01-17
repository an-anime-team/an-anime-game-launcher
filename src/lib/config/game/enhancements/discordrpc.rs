use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::prelude::Fsr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRpc {
    pub enabled: bool,
    pub large_image_key: String,
    pub app_id: u64,
    pub description: String,
    pub state: String,
}

impl Default for DiscordRpc {
    fn default() -> Self {
        Self {
            enabled: true,
            large_image_key: "gi-icon".to_string(),
            app_id: 901534333360304168,
            description: "Bullying Paimon".to_string(),
            state: "In the weeb game".to_string(),
        }
    }
}

impl From<&JsonValue> for DiscordRpc {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();
        Self {
            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            },

            description: match value.get("description") {
                Some(value) => value.as_str().unwrap_or(&default.description).to_string(),
                None => default.description
            },

            state: match value.get("state") {
                Some(value) => value.as_str().unwrap_or(&default.state).to_string(),
                None => default.state
            },
            
            large_image_key: match value.get("large_image_key"){
                Some(value) => value.as_str().unwrap_or(&default.large_image_key).to_string(),
                None => default.large_image_key
            },
            app_id: match value.get("app_id"){
                Some(value) => value.as_u64().unwrap_or(default.app_id),
                None => default.app_id
            },
        }
    }
}


impl DiscordRpc
{
    pub fn toggle(&self)
    {
        println!("[Debug] RPC state changed!");
        if self.enabled
        {
            todo!();
        }
        else 
        {
            todo!();
        }
    }
}