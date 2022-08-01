use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::{Error, ErrorKind, Write};
use std::process::{Command, Stdio};

use serde::{Serialize, Deserialize};

use super::consts::*;
use super::wine::{
    Version as WineVersion,
    List as WineList
};

mod hud;
mod wine_sync;
mod wine_lang;

pub use hud::HUD;
pub use wine_sync::WineSync;
pub use wine_lang::WineLang;

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

                match serde_json::from_str::<Config>(&json) {
                    Ok(config) => {
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
    /// Try to get a path to the wine executable based on `game.wine.builds` and `game.wine.selected`
    /// 
    /// Returns `Some("wine")` if:
    /// 1) `game.wine.selected = None`
    /// 2) wine installed and available in system
    pub fn try_get_wine_executable(&self) -> Option<String> {
        match &self.game.wine.selected {
            Some(selected) => {
                // Most of wine builds
                let path = format!("{}/{}/bin/wine", &self.game.wine.builds, &selected);

                if Path::new(&path).exists() {
                    return Some(path);
                }

                // Proton-based builds
                let path = format!("{}/{}/files/bin/wine", &self.game.wine.builds, &selected);

                if Path::new(&path).exists() {
                    return Some(path);
                }

                // ????
                None
            },
            None => match Command::new("wine").stdout(Stdio::null()).stderr(Stdio::null()).output() {
                Ok(output) => if output.status.success() { Some(String::from("wine")) } else { None },
                Err(_) => None
            }
        }
    }

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    pub language: String,
    pub temp: Option<String>
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            language: String::from("en-us"),
            temp: launcher_dir()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub path: String,
    pub servers: Vec<String>,
    pub root: bool
}

impl Default for Patch {
    fn default() -> Self {
        Self {
            path: match launcher_dir() {
                Some(dir) => format!("{}/patch", dir),
                None => String::new()
            },
            servers: vec![
                "https://notabug.org/Krock/dawn".to_string(),
                "https://dev.kaifa.ch/Maroxy/dawn".to_string()
            ],
            root: true
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub path: String,
    pub voices: Vec<String>,
    pub wine: Wine,
    pub dxvk: Dxvk,
    pub enhancements: Enhancements,
    pub environment: HashMap<String, String>
}

impl Default for Game {
    fn default() -> Self {
        Self {
            path: match launcher_dir() {
                Some(dir) => format!("{}/game/drive_c/Program Files/Genshin Impact", dir),
                None => String::new()
            },
            voices: vec![
                String::from("en-us")
            ],
            wine: Wine::default(),
            dxvk: Dxvk::default(),
            enhancements: Enhancements::default(),
            environment: HashMap::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wine {
    pub prefix: String,
    pub builds: String,
    pub selected: Option<String>,
    pub sync: WineSync,
    pub language: WineLang
}

impl Default for Wine {
    fn default() -> Self {
        Self {
            prefix: match launcher_dir() {
                Some(dir) => format!("{}/game", dir),
                None => String::new()
            },
            builds: match launcher_dir() {
                Some(dir) => format!("{}/runners", dir),
                None => String::new()
            },
            selected: None,
            sync: WineSync::default(),
            language: WineLang::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dxvk {
    pub builds: String,
    pub selected: Option<String>
}

impl Default for Dxvk {
    fn default() -> Self {
        Self {
            builds: match launcher_dir() {
                Some(dir) => format!("{}/dxvks", dir),
                None => String::new()
            },
            selected: None
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Enhancements {
    pub fsr: Fsr,
    pub gamemode: bool,
    pub hud: HUD
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Fsr {
    pub strength: u32,
    pub enabled: bool
}

impl Default for Fsr {
    fn default() -> Self {
        Self {
            strength: 2,
            enabled: false
        }
    }
}

impl Fsr {
    /// Get environment variables corresponding to used amd fsr options
    pub fn get_env_vars(&self) -> HashMap<&str, String> {
        if self.enabled {
            HashMap::from([
                ("WINE_FULLSCREEN_FSR", String::from("1")),
                ("WINE_FULLSCREEN_FSR_STRENGTH", self.strength.to_string())
            ])
        }
        
        else {
            HashMap::new()
        }
    }
}
