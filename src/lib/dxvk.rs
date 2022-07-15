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

    /// List only downloaded DXVK versions in some specific folder
    pub fn list_downloaded<T: ToString>(folder: T) -> std::io::Result<List> {
        let mut vanilla = Vec::new();
        let mut r#async = Vec::new();

        let list = Self::get()?;

        for entry in std::fs::read_dir(folder.to_string())? {
            let name = entry?.file_name();

            for (i, versions) in [&list.vanilla, &list.r#async].into_iter().enumerate() {
                for version in versions {
                    if name == version.name.as_str() {
                        match i {
                            0 => vanilla.push(version.clone()),
                            1 => r#async.push(version.clone()),
                            _ => unreachable!()
                        }

                        break;
                    }
                }
            }
        }

        Ok(List {
            vanilla,
            r#async
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub version: String,
    pub uri: String,
    pub recommended: bool
}

impl Version {
    pub fn is_downloaded_in<T: ToString>(&self, folder: T) -> bool {
        std::path::Path::new(&format!("{}/{}", folder.to_string(), self.name)).exists()
    }
}
