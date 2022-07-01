use std::collections::HashMap;
use std::{fs::File, io::Read};
use std::path::Path;
use std::io::{Error, ErrorKind, Write};

use serde::{Serialize, Deserialize};

use super::consts::*;

mod wine_hud;
mod wine_sync;

pub use wine_hud::WineHUD;
pub use wine_sync::WineSync;

pub fn get() -> Result<Config, Error> {
    match config_file() {
        Some(path) => {
            // Try to read config if the file exists
            if Path::new(&path).exists() {
                let mut file = File::open(path)?;
                let mut json = String::new();

                file.read_to_string(&mut json)?;

                match serde_json::from_str::<Config>(&json) {
                    Ok(json) => Ok(json),
                    Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to decode data from json format: {}", err.to_string())))
                }
            }

            // Otherwise create default config file
            else {
                update(Config::default())?;

                Ok(Config::default())
            }
        },
        None => Err(Error::new(ErrorKind::NotFound, format!("Failed to get config file path")))
    }
}

pub fn update(config: Config) -> Result<(), Error> {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub launcher: Launcher,
    pub game: Game,
    pub patch: Patch
}

impl Config {
    /// Try to get a path to the wine executable based on `game.wine.builds` and `game.wine.selected`
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
            None => None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    pub language: String
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            language: String::from("en-us")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub path: String,
    pub servers: Vec<String>
}

impl Default for Patch {
    fn default() -> Self {
        Self {
            path: match launcher_dir() {
                Some(dir) => format!("{}/patch", dir),
                None => String::new()
            },
            servers: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub path: String,
    pub voices: Vec<String>,
    pub wine: Wine,
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
    pub sync: WineSync
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
            sync: WineSync::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Enhancements {
    pub fsr: Fsr,
    pub gamemode: bool,
    pub hud: WineHUD
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Fsr {
    pub strength: u32,
    pub enabled: bool
}

impl Default for Fsr {
    fn default() -> Self {
        Self {
            strength: 3,
            enabled: false
        }
    }
}
