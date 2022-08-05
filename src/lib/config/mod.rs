use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::{Error, ErrorKind, Write};

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::lib;
use super::consts::*;
use super::wine::{
    Version as WineVersion,
    List as WineList
};

pub mod launcher;
pub mod game;
pub mod patch;
pub mod resolution;

pub mod prelude {
    pub use super::launcher::prelude::*;
    pub use super::game::prelude::*;

    pub use super::patch::Patch;
    pub use super::resolution::Resolution;
}

use prelude::*;

static mut CONFIG: Option<Config> = None; 

/// Get config data
/// 
/// This method will load config from file once and store it into the memory.
/// If you know that the config file was updated - you should run `get_raw` method
/// that always loads config directly from the file. This will also update in-memory config
pub fn get() -> Result<Config, Error> {
    unsafe {
        match &CONFIG {
            Some(config) => Ok(config.clone()),
            None => get_raw()
        }
    }
}

/// Get config data
/// 
/// This method will always load data directly from the file and update in-memory config
pub fn get_raw() -> Result<Config, Error> {
    match config_file() {
        Some(path) => {
            // Try to read config if the file exists
            if Path::new(&path).exists() {
                let mut file = File::open(path)?;
                let mut json = String::new();

                file.read_to_string(&mut json)?;

                match serde_json::from_str(&json) {
                    Ok(config) => {
                        let config = Config::from(&config);

                        unsafe {
                            CONFIG = Some(config.clone());
                        }

                        Ok(config)
                    },
                    Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to decode data from json format: {}", err.to_string())))
                }
            }

            // Otherwise create default config file
            else {
                update_raw(Config::default())?;

                Ok(Config::default())
            }
        },
        None => Err(Error::new(ErrorKind::NotFound, format!("Failed to get config file path")))
    }
}

/// Update in-memory config data
/// 
/// Use `update_raw` if you want to update config file itself
pub fn update(config: Config) {
    unsafe {
        CONFIG = Some(config);
    }
}

/// Update config file
/// 
/// This method will also update in-memory config data
pub fn update_raw(config: Config) -> Result<(), Error> {
    update(config.clone());

    match config_file() {
        Some(path) => {
            let mut file = File::create(&path)?;

            match serde_json::to_string_pretty(&config) {
                Ok(json) => {
                    file.write_all(&mut json.as_bytes())?;

                    Ok(())
                },
                Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to encode data into json format: {}", err.to_string())))
            }
        },
        None => Err(Error::new(ErrorKind::NotFound, format!("Failed to get config file path")))
    }
}

/// Update config file from the in-memory saved config
pub fn flush() -> Result<(), Error> {
    unsafe {
        match &CONFIG {
            Some(config) => update_raw(config.clone()),
            None => Err(Error::new(ErrorKind::Other, "Config wasn't loaded into the memory"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub launcher: Launcher,
    pub game: Game,
    pub patch: Patch
}

impl Config {
    pub fn try_get_selected_wine_info(&self) -> Option<WineVersion> {
        match &self.game.wine.selected {
            Some(selected) => {
                match WineList::get() {
                    Ok(list) => {
                        for group in list {
                            for version in group.versions {
                                if &version.name == selected {
                                    return Some(version.clone());
                                }
                            }
                        }

                        None
                    },
                    Err(_) => None
                }
            },
            None => None
        }
    }

    /// Try to get a path to the wine64 executable based on `game.wine.builds` and `game.wine.selected`
    /// 
    /// Returns `Some("wine64")` if:
    /// 1) `game.wine.selected = None`
    /// 2) wine64 installed and available in system
    pub fn try_get_wine_executable(&self) -> Option<String> {
        match self.try_get_selected_wine_info() {
            Some(selected) => Some(format!("{}/{}/{}", &self.game.wine.builds, selected.name, selected.files.wine64)),
            None => {
                if lib::is_available("wine64") {
                    Some(String::from("wine64"))
                } else {
                    None
                }
            }
        }
    }
}

impl From<&JsonValue> for Config {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            launcher: match value.get("launcher") {
                Some(value) => Launcher::from(value),
                None => default.launcher
            },

            game: match value.get("game") {
                Some(value) => Game::from(value),
                None => default.game
            },

            patch: match value.get("patch") {
                Some(value) => Patch::from(value),
                None => default.patch
            }
        }
    }
}
