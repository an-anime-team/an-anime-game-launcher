use serde::{Serialize, Deserialize};

use std::io::{Error, ErrorKind};

use lazy_static::lazy_static;
use regex::Regex;

use crate::lib::config;

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

    pub fn apply<T: ToString>(&self, dxvks_folder: T, prefix_path: T) -> std::io::Result<String> {
        let apply_path = format!("{}/{}/setup_dxvk.sh", dxvks_folder.to_string(), self.name);
        let config = config::get()?;

        match config.try_get_selected_wine_info() {
            Some(wine) => {
                let wine_path = format!("{}/{}/{}", &config.game.wine.builds, wine.name, wine.files.wine);
                let wineserver_path = format!("{}/{}/{}", &config.game.wine.builds, wine.name, wine.files.wineserver);
                let wineboot_path = format!("{}/{}/{}", &config.game.wine.builds, wine.name, wine.files.wineboot);

                let mut apply_script = std::fs::read_to_string(&apply_path)?;

                lazy_static! {
                    static ref WINE: Regex = Regex::new("wine=\".*\"").unwrap();
                    static ref WINE64: Regex = Regex::new("wine64=\".*\"").unwrap();
                    static ref WINEBOOT: Regex = Regex::new("wineboot=\".*\"").unwrap();
                }

                // Update wine paths
                apply_script = WINE.replace_all(&apply_script, &format!("wine=\"{}\"", &wine_path)).to_string();
                apply_script = WINE64.replace_all(&apply_script, &format!("wine64=\"{}\"", &wine_path)).to_string();
                apply_script = WINEBOOT.replace_all(&apply_script, &format!("wineboot=\"{}\"", &wineboot_path)).to_string();

                // Old GE builds return specific --version output which can break
                // DXVK installation script
                apply_script = apply_script.replace("grep wine", "grep \"wine\\|GE\"");

                std::fs::write(&apply_path, apply_script)?;

                let output = std::process::Command::new("bash")
                    .arg(&apply_path)
                    .arg("install")
                    .env("WINE", wine_path)
                    .env("WINESERVER", wineserver_path)
                    .env("WINEPREFIX", prefix_path.to_string())
                    .output()?;

                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).unwrap())
                }

                else {
                    Err(Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr)))
                }
            },
            None => Err(Error::new(ErrorKind::Other, "Wine is not selected"))
        }
    }
}
