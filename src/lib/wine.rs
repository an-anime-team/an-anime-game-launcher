use serde::{Serialize, Deserialize};

const LIST: &str = include_str!("../../assets/wine.json");

pub struct List;

impl List {
    pub fn get() -> Result<Vec<Group>, serde_json::Error> {
        Ok(serde_json::from_str(LIST)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub title: String,
    pub subtitle: Option<String>,
    pub runners: Vec<Version>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub family: String,
    pub name: String,
    pub title: String,
    pub uri: String,
    pub files: Files,
    pub recommended: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Files {
    pub wine: String,
    pub wineserver: String,
    pub winecfg: String
}
