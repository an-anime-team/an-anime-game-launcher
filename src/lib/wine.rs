use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use wincompatlib::prelude::*;

lazy_static::lazy_static! {
    static ref GROUPS: Vec<Group> = vec![
        Group {
            title: String::from("Wine-GE-Proton"),
            subtitle: None,
            versions: serde_json::from_str(include_str!("../../components/wine/wine-ge-proton.json")).unwrap()
        },
        Group {
            title: String::from("GE-Proton"),
            subtitle: Some(String::from("This version includes its own DXVK builds and you can use DXVK_ASYNC variable")),
            versions: serde_json::from_str(include_str!("../../components/wine/ge-proton.json")).unwrap()
        },
        Group {
            title: String::from("Soda"),
            subtitle: Some(String::from("New runner based on Valve’s Wine, with patches from Proton, TKG and GE. Developed by Bottles")),
            versions: serde_json::from_str(include_str!("../../components/wine/soda.json")).unwrap()
        },
        Group {
            title: String::from("Lutris"),
            subtitle: None,
            versions: serde_json::from_str(include_str!("../../components/wine/lutris.json")).unwrap()
        }
    ];
}

pub struct List;

impl List {
    pub fn get() -> Vec<Group> {
        GROUPS.clone()
    }

    /// List only downloaded wine versions in some specific folder
    pub fn list_downloaded<T: ToString>(folder: T) -> std::io::Result<Vec<Version>> {
        let mut downloaded = Vec::new();

        let list = Self::get();

        for entry in std::fs::read_dir(folder.to_string())? {
            let name = entry?.file_name();

            for group in &list {
                for version in &group.versions {
                    if name == version.name.as_str() {
                        downloaded.push(version.clone());

                        break;
                    }
                }
            }
        }

        downloaded.sort_by(|a, b| b.name.partial_cmp(&a.name).unwrap());

        Ok(downloaded)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub title: String,
    pub subtitle: Option<String>,
    pub versions: Vec<Version>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub title: String,
    pub uri: String,
    pub files: Files,
    pub recommended: bool
}

impl Version {
    pub fn latest() -> Result<Self, serde_json::Error> {
        Ok(List::get()[0].versions[0].clone())
    }

    pub fn is_downloaded_in<T: Into<PathBuf>>(&self, folder: T) -> bool {
        folder.into().join(&self.name).exists()
    }

    pub fn to_wine(&self) -> Wine {
        Wine::new(
            &self.files.wine64,
            None,
            Some(WineArch::Win64),
            Some(&self.files.wineboot),
            Some(&self.files.wineserver),
            WineLoader::Current
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Files {
    pub wine: String,
    pub wine64: String,
    pub wineserver: String,
    pub wineboot: String,
    pub winecfg: String
}
