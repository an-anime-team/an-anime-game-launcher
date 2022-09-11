use anime_game_core::installer::downloader::Downloader;

use crate::lib::config::game::enhancements::fps_unlocker::config::Config as FpsUnlockerConfig;

pub mod config_schema;

const LATEST_INFO: (&str, &str) = (
    "6040a6f0be5dbf4d55d6b129cad47b5b",
    "https://github.com/34736384/genshin-fps-unlock/releases/download/v2.0.0/unlockfps_clr.exe"
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FpsUnlocker {
    dir: String
}

impl FpsUnlocker {
    /// Get FpsUnlocker from its containment directory
    /// 
    /// Returns
    /// - `Err(..)` if failed to read `unlocker.exe` file
    /// - `Ok(None)` if version is not latest
    /// - `Ok(..)` if version is latest
    pub fn from_dir<T: ToString>(dir: T) -> anyhow::Result<Option<Self>> {
        let hash = format!("{:x}", md5::compute(std::fs::read(format!("{}/unlocker.exe", dir.to_string()))?));

        if hash == LATEST_INFO.0 {
            Ok(Some(Self { dir: dir.to_string() }))
        } else {
            Ok(None)
        }
    }

    /// Download FPS unlocker to specified directory
    pub fn download<T: ToString>(dir: T) -> anyhow::Result<Self> {
        let mut downloader = Downloader::new(LATEST_INFO.1)?;

        match downloader.download_to(format!("{}/unlocker.exe", dir.to_string()), |_, _| {}) {
            Ok(_) => Ok(Self {
                dir: dir.to_string()
            }),
            Err(err) => {
                let err: std::io::Error = err.into();

                Err(err.into())
            }
        }
    }

    pub fn get_binary(&self) -> String {
        format!("{}/unlocker.exe", self.dir)
    }

    pub fn dir(&self) -> &str {
        self.dir.as_str()
    }

    /// Generate and save FPS unlocker config file to the game's directory
    pub fn update_config(&self, config: FpsUnlockerConfig) -> anyhow::Result<()> {
        let config = config_schema::ConfigSchema::from_config(config);

        Ok(std::fs::write(
            format!("{}/fps_config.json", self.dir),
            config.json()?
        )?)
    }
}
