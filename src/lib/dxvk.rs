use serde::{Serialize, Deserialize};

use std::process::Output;
use std::path::PathBuf;

use lazy_static::lazy_static;

use wincompatlib::prelude::*;

use crate::lib::config;

lazy_static! {
    static ref GROUPS: Vec<Group> = vec![
        Group {
            title: String::from("Vanilla"),
            subtitle: None,
            versions: serde_json::from_str(include_str!("../../components/dxvk/vanilla.json")).unwrap()
        },
        Group {
            title: String::from("Async"),
            subtitle: Some(String::from("This version is not recommended for usage as can lead to anti-cheat detection. Automatically uses DXVK_ASYNC=1")),
            versions: serde_json::from_str(include_str!("../../components/dxvk/async.json")).unwrap()
        }
    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List;

impl List {
    pub fn get() -> Vec<Group> {
        GROUPS.clone()
    }

    /// List only downloaded DXVK versions in some specific folder
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

        Ok(downloaded)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub title: String,
    pub subtitle: Option<String>,
    pub versions: Vec<Version>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub version: String,
    pub uri: String,
    pub recommended: bool
}

impl Version {
    pub fn latest() -> Result<Self, serde_json::Error> {
        Ok(List::get()[0].versions[0].clone())
    }

    pub fn is_downloaded_in<T: ToString>(&self, folder: T) -> bool {
        std::path::Path::new(&format!("{}/{}", folder.to_string(), self.name)).exists()
    }

    pub fn apply<T: ToString>(&self, dxvks_folder: T, prefix_path: T) -> anyhow::Result<Output> {
        let apply_path = format!("{}/{}/setup_dxvk.sh", dxvks_folder.to_string(), self.name);
        let config = config::get()?;

        let (wine_path, wineserver_path, wineboot_path) = match config.try_get_selected_wine_info() {
            Some(wine) => {
                let wine_path = format!("{}/{}/{}", &config.game.wine.builds, wine.name, wine.files.wine64);
                let wineserver_path = format!("{}/{}/{}", &config.game.wine.builds, wine.name, wine.files.wineserver);
                let wineboot_path = format!("{}/{}/{}", &config.game.wine.builds, wine.name, wine.files.wineboot);

                (wine_path, wineserver_path, wineboot_path)
            },
            None => (String::from("wine64"), String::from("wineserver"), String::from("wineboot"))
        };

        let result = Dxvk::install(
            PathBuf::from(apply_path),
            PathBuf::from(prefix_path.to_string()),
            PathBuf::from(&wine_path),
            PathBuf::from(wine_path),
            PathBuf::from(wineboot_path),
            PathBuf::from(wineserver_path)
        );

        match result {
            Ok(output) => Ok(output),
            Err(err) => Err(err.into())
        }
    }
}
