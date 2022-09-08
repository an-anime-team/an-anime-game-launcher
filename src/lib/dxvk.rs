use serde::{Serialize, Deserialize};

use std::io::{Error, ErrorKind};
use std::process::{Command, Output};

use lazy_static::lazy_static;
use regex::Regex;

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

    pub fn apply<T: ToString>(&self, dxvks_folder: T, prefix_path: T) -> std::io::Result<Output> {
        let apply_path = format!("{}/{}/setup_dxvk.sh", dxvks_folder.to_string(), self.name);
        let config = config::get()?;

        match config.try_get_selected_wine_info() {
            Some(wine) => {
                let wine_path = format!("{}/{}/{}", &config.game.wine.builds, wine.name, wine.files.wine64);
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

                // Use wine64 to update wine prefix instead of running wineboot
                // so we can get rid of 32bit support
                apply_script = apply_script.replace("$wineboot -u", "\"$wine64\" -u");

                // Fix issues related to spaces in paths to the runners folder
                apply_script = apply_script.replace("which $wineboot", "which \"$wineboot\"");
                apply_script = apply_script.replace("$wine --version", "\"$wine\" --version");
                apply_script = apply_script.replace("$wine64 winepath", "\"$wine64\" winepath");
                apply_script = apply_script.replace("$wine winepath", "\"$wine\" winepath");
                apply_script = apply_script.replace("$wine reg", "\"$wine\" reg");

                // Old GE builds return specific --version output which can break
                // DXVK installation script
                apply_script = apply_script.replace("grep wine", "grep \"wine\\|GE\"");

                std::fs::write(&apply_path, apply_script)?;

                let output = Command::new("bash")
                    .arg(&apply_path)
                    .arg("install")
                    .env("WINEARCH", "win64")
                    .env("WINESERVER", wineserver_path)
                    .env("WINEPREFIX", prefix_path.to_string())
                    .output()?;

                if output.status.success() {
                    Ok(output)
                }

                else {
                    Err(Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr)))
                }
            },
            None => Err(Error::new(ErrorKind::Other, "Wine is not selected"))
        }
    }
}
