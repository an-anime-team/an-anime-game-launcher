use serde::{Serialize, Deserialize};

const LIST: &str = include_str!("../../assets/dxvk.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub vanilla: Vec<Version>,
    pub r#async: Vec<Version>
}

impl List {
    pub fn get() -> Result<Self, serde_json::Error> {
        Ok(serde_json::from_str(LIST)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub uri: String,
    pub recommended: bool
}
