use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WineLang {
    System,
    English,
    Russian,
    German,
    Portuguese,
    Polish,
    French,
    Spanish,
    Chinese,
    Japanese,
    Korean
}

impl Default for WineLang {
    fn default() -> Self {
        Self::System
    }
}

impl From<&JsonValue> for WineLang {
    fn from(value: &JsonValue) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for WineLang {
    fn into(self) -> u32 {
        for (i, lang) in Self::list().into_iter().enumerate() {
            if lang == self {
                return i as u32;
            }
        }

        unreachable!()
    }
}

impl WineLang {
    pub fn list() -> Vec<Self> {
        vec![
            Self::System,
            Self::English,
            Self::Russian,
            Self::German,
            Self::Portuguese,
            Self::Polish,
            Self::French,
            Self::Spanish,
            Self::Chinese,
            Self::Japanese,
            Self::Korean
        ]
    }

    pub fn get_model() -> gtk::StringList {
        let model = gtk::StringList::new(&[]);

        for lang in Self::list() {
            model.append(&lang.to_string());
        }

        model
    }

    /// Get environment variables corresponding to used wine language
    pub fn get_env_vars(&self) -> HashMap<&str, &str> {
        HashMap::from([("LANG", match self {
            Self::System => return HashMap::new(),

            Self::English    => "en_US.UTF8",
            Self::Russian    => "ru_RU.UTF8",
            Self::German     => "de_DE.UTF8",
            Self::Portuguese => "pt_PT.UTF8",
            Self::Polish     => "pl_PL.UTF8",
            Self::French     => "fr_FR.UTF8",
            Self::Spanish    => "es_ES.UTF8",
            Self::Chinese    => "zh_CN.UTF8",
            Self::Japanese   => "ja_JP.UTF8",
            Self::Korean     => "ko_KR.UTF8"
        })])
    }
}

impl std::fmt::Display for WineLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
