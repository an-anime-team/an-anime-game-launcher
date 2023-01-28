use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DiscordRpcIcons {
    Launcher,
    Game,

    Amber1,
    Amber2,

    Beidou,

    Klee1,
    Klee2,
    Klee3,

    Raiden,

    YaeMiko1,
    YaeMiko2,

    LiYue,

    Inazuma1,
    Inazuma2,
    Inazuma3,
    Inazuma4,
    Inazuma5
}

impl DiscordRpcIcons {
    pub fn list() -> Vec<Self> {
        vec![
            Self::Launcher,
            Self::Game,

            Self::Amber1,
            Self::Amber2,

            Self::Beidou,

            Self::Klee1,
            Self::Klee2,
            Self::Klee3,

            Self::Raiden,

            Self::YaeMiko1,
            Self::YaeMiko2,

            Self::LiYue,

            Self::Inazuma1,
            Self::Inazuma2,
            Self::Inazuma3,
            Self::Inazuma4,
            Self::Inazuma5
        ]
    }

    pub fn get_model() -> gtk::StringList {
        let model = gtk::StringList::new(&[]);

        for icon in Self::list() {
            model.append(&icon.to_string());
        }

        model
    }

    /// Get Discord RPC icon name
    pub fn get_icon_name(&self) -> &'static str {
        match self {
            Self::Launcher => "launcher",
            Self::Game => "gi-icon",

            Self::Amber1 => "artgame",
            Self::Amber2 => "artgame3",

            Self::Beidou => "beidougame",

            Self::Klee1 => "kleegame",
            Self::Klee2 => "kleegame2",
            Self::Klee3 => "artgame2",

            Self::Raiden => "baal1",

            Self::YaeMiko1 => "yaemiko1",
            Self::YaeMiko2 => "yaemiko2",

            Self::LiYue => "liyuegame",

            Self::Inazuma1 => "inazuma1",
            Self::Inazuma2 => "inazuma2",
            Self::Inazuma3 => "inazuma3",
            Self::Inazuma4 => "inazuma4",
            Self::Inazuma5 => "inazuma5"
        }
    }
}

impl Default for DiscordRpcIcons {
    fn default() -> Self {
        Self::Launcher
    }
}

impl From<&JsonValue> for DiscordRpcIcons {
    fn from(value: &JsonValue) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}

impl std::fmt::Display for DiscordRpcIcons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRpc {
    pub app_id: u64,
    pub enabled: bool,

    pub title: String,
    pub subtitle: String,
    pub icon: DiscordRpcIcons
}

impl Default for DiscordRpc {
    fn default() -> Self {
        Self {
            app_id: 901534333360304168,
            enabled: false,

            title: String::from("Researching the world"),
            subtitle: String::from("of Teyvat"),
            icon: DiscordRpcIcons::default()
        }
    }
}

impl From<&JsonValue> for DiscordRpc {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            app_id: match value.get("app_id") {
                Some(value) => value.as_u64().unwrap_or(default.app_id),
                None => default.app_id
            },

            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            },

            title: match value.get("title") {
                Some(value) => value.as_str().unwrap_or(&default.title).to_string(),
                None => default.title
            },

            subtitle: match value.get("subtitle") {
                Some(value) => value.as_str().unwrap_or(&default.subtitle).to_string(),
                None => default.subtitle
            },

            icon: match value.get("icon") {
                Some(value) => value.into(),
                None => default.icon
            }
        }
    }
}
