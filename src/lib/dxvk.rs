use serde::{Serialize, Deserialize};

use std::path::PathBuf;

use lazy_static::lazy_static;

use wincompatlib::prelude::*;

use crate::lib::config;

lazy_static! {
    static ref GROUPS: Vec<Group> = vec![
        Group {
            title: String::from("Vanilla"),
            subtitle: None,
            versions: serde_json::from_str::<Vec<Version>>(include_str!("../../components/dxvk/vanilla.json")).unwrap().into_iter().take(12).collect()
        },
        Group {
            title: String::from("Async"),
            subtitle: Some(String::from("This version is not recommended for usage as can lead to anti-cheat detection. Automatically uses DXVK_ASYNC=1")),
            versions: serde_json::from_str::<Vec<Version>>(include_str!("../../components/dxvk/async.json")).unwrap().into_iter().take(12).collect()
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
    pub fn list_downloaded<T: Into<PathBuf>>(folder: T) -> std::io::Result<Vec<Version>> {
        let mut downloaded = Vec::new();

        let list = Self::get();

        for entry in std::fs::read_dir(folder.into())? {
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

    pub fn is_downloaded_in<T: Into<PathBuf>>(&self, folder: T) -> bool {
        folder.into().join(&self.name).exists()
    }

    pub fn install<T: Into<PathBuf>>(&self, dxvks_folder: T, prefix_path: T, params: InstallParams) -> anyhow::Result<()> {
        let config = config::get()?;

        let wine = match config.try_get_selected_wine_info() {
            Some(wine) => {
                let wine_folder = config.game.wine.builds.join(wine.name);

                let wine_path = wine_folder.join(wine.files.wine64);
                let wineserver_path = wine_folder.join(wine.files.wineserver);
                let wineboot_path = wine_folder.join(wine.files.wineboot);

                Wine::from_binary(wine_path)
                    .with_server(wineserver_path)
                    .with_boot(wineboot_path)
                    .with_loader(WineLoader::Current)
                    .with_arch(WineArch::Win64)
                    .with_prefix(prefix_path)
            },
            None => Wine::default().with_prefix(prefix_path)
        };
        
        let result = Dxvk::install(
            &wine,
            dxvks_folder.into().join(&self.name),
            params
        );

        match result {
            Ok(()) => Ok(()),
            Err(err) => Err(anyhow::anyhow!(err))
        }
    }
}
