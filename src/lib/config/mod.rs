use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::{Error, ErrorKind, Write};

use serde::{Serialize, Deserialize};

use crate::lib;
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
            None => {
                if lib::is_available("wine") {
                    Some(String::from("wine"))
                } else {
                    None
                }
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

    pub fn get_gamescope_command(&self) -> Option<String> {
        // https://github.com/bottlesdevs/Bottles/blob/b908311348ed1184ead23dd76f9d8af41ff24082/src/backend/wine/winecommand.py#L478
        if self.game.enhancements.gamescope.enabled {
            let mut gamescope = String::from("gamescope");

            // Set window type
            match self.game.enhancements.gamescope.window_type {
                WindowType::Borderless => gamescope += " -b",
                WindowType::Fullscreen => gamescope += " -f"
            }

            // Set game width
            if self.game.enhancements.gamescope.game.width > 0 {
                gamescope += &format!(" -w {}", self.game.enhancements.gamescope.game.width);
            }

            // Set game height
            if self.game.enhancements.gamescope.game.height > 0 {
                gamescope += &format!(" -h {}", self.game.enhancements.gamescope.game.height);
            }

            // Set gamescope width
            if self.game.enhancements.gamescope.gamescope.width > 0 {
                gamescope += &format!(" -W {}", self.game.enhancements.gamescope.gamescope.width);
            }

            // Set gamescope height
            if self.game.enhancements.gamescope.gamescope.height > 0 {
                gamescope += &format!(" -H {}", self.game.enhancements.gamescope.gamescope.height);
            }

            // Set focused framerate limit
            if self.game.enhancements.gamescope.framerate.focused > 0 {
                gamescope += &format!(" -r {}", self.game.enhancements.gamescope.framerate.focused);
            }

            // Set unfocused framerate limit
            if self.game.enhancements.gamescope.framerate.unfocused > 0 {
                gamescope += &format!(" -o {}", self.game.enhancements.gamescope.framerate.unfocused);
            }

            // Set integer scaling
            if self.game.enhancements.gamescope.integer_scaling {
                gamescope += " -n";
            }

            // Set FSR support
            if self.game.enhancements.fsr.enabled {
                gamescope += " -U";
            }

            Some(gamescope)
        }

        else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    pub language: String,
    pub temp: Option<String>,
    pub repairer: Repairer
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            language: String::from("en-us"),
            temp: launcher_dir(),
            repairer: Repairer::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repairer {
    pub threads: u8,
    pub fast: bool
}

impl Default for Repairer {
    fn default() -> Self {
        Self {
            threads: 4,
            fast: false
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

            // Disable root requirement for patching if we're running launcher in flatpak
            root: !Path::new("/.flatpak-info").exists()
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
    pub environment: HashMap<String, String>,
    pub command: Option<String>
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
            environment: HashMap::new(),
            command: None
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
    pub hud: HUD,
    pub gamescope: Gamescope
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gamescope {
    pub enabled: bool,
    pub game: Size,
    pub gamescope: Size,
    pub framerate: Framerate,
    pub integer_scaling: bool,
    pub window_type: WindowType
}

impl Default for Gamescope {
    fn default() -> Self {
        Self {
            enabled: false,
            game: Size::default(),
            gamescope: Size::default(),
            framerate: Framerate::default(),
            integer_scaling: true,
            window_type: WindowType::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Framerate {
    pub focused: u16,
    pub unfocused: u16
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WindowType {
    Borderless,
    Fullscreen
}

impl Default for WindowType {
    fn default() -> Self {
        Self::Borderless
    }
}
