use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib::consts::launcher_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub path: PathBuf,
    pub servers: Vec<String>,
    pub root: bool
}

impl Default for Patch {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            path: launcher_dir.join("patch"),
            servers: vec![
                "https://notabug.org/Krock/dawn".to_string(),
                "https://codespace.gay/Maroxy/dawnin".to_string()
            ],

            // Disable root requirement for patching if we're running launcher in flatpak
            root: !Path::new("/.flatpak-info").exists()
        }
    }
}

impl From<&JsonValue> for Patch {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            path: match value.get("path") {
                Some(value) => match value.as_str() {
                    Some(value) => PathBuf::from(value),
                    None => default.path
                },
                None => default.path
            },

            servers: match value.get("servers") {
                Some(value) => match value.as_array() {
                    Some(values) => {
                        let mut servers = Vec::new();

                        for value in values {
                            if let Some(server) = value.as_str() {
                                servers.push(server.to_string());
                            }
                        }

                        servers
                    },
                    None => default.servers
                },
                None => default.servers
            },

            root: match value.get("root") {
                Some(value) => value.as_bool().unwrap_or(default.root),
                None => default.root
            }
        }
    }
}
