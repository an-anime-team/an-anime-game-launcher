use std::path::PathBuf;

use anime_game_core::installer::downloader::Downloader;

use crate::lib::config::game::enhancements::fps_unlocker::config::Config as FpsUnlockerConfig;

pub mod config_schema;

const LATEST_INFO: (&str, &str) = (
    "6040a6f0be5dbf4d55d6b129cad47b5b",
    "https://github.com/34736384/genshin-fps-unlock/releases/download/v2.0.0/unlockfps_clr.exe"
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpsUnlocker {
    dir: PathBuf
}

impl FpsUnlocker {
    /// Get FpsUnlocker from its containment directory
    /// 
    /// Returns
    /// - `Err(..)` if failed to read `unlocker.exe` file
    /// - `Ok(None)` if version is not latest
    /// - `Ok(..)` if version is latest
    pub fn from_dir<T: Into<PathBuf>>(dir: T) -> anyhow::Result<Option<Self>> {
        let dir = dir.into();

        let hash = format!("{:x}", md5::compute(std::fs::read(dir.join("unlocker.exe"))?));

        Ok(if hash == LATEST_INFO.0 {
            Some(Self { dir })
        } else {
            None
        })
    }

    /// Download FPS unlocker to specified directory
    pub fn download<T: Into<PathBuf>>(dir: T) -> anyhow::Result<Self> {
        let mut downloader = Downloader::new(LATEST_INFO.1)?;

        let dir = dir.into();

        // Create FPS unlocker folder if needed
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        match downloader.download_to(dir.join("unlocker.exe"), |_, _| {}) {
            Ok(_) => Ok(Self {
                dir
            }),
            Err(err) => {
                let err: std::io::Error = err.into();

                Err(err.into())
            }
        }
    }

    pub fn get_binary(&self) -> PathBuf {
        Self::get_binary_in(&self.dir)
    }

    pub fn get_binary_in<T: Into<PathBuf>>(dir: T) -> PathBuf {
        dir.into().join("unlocker.exe")
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    /// Generate and save FPS unlocker config file to the game's directory
    pub fn update_config(&self, config: FpsUnlockerConfig) -> anyhow::Result<()> {
        let config = config_schema::ConfigSchema::from_config(config);

        Ok(std::fs::write(
            self.dir.join("fps_config.json"),
            config.json()?
        )?)
    }
}
