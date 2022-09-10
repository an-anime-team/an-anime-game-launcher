use anime_game_core::prelude::*;
use anime_game_core::genshin::prelude::*;

use crate::lib::consts;
use crate::lib::config;
use crate::lib::wine_prefix::WinePrefix;

#[derive(Debug, Clone)]
pub enum LauncherState {
    Launch,

    /// Always contains `VersionDiff::Predownload`
    PredownloadAvailable {
        game: VersionDiff,
        voices: Vec<VersionDiff>
    },

    PatchAvailable(Patch),

    WineNotInstalled,
    PrefixNotExists,

    // Always contains `VersionDiff::Diff`
    VoiceUpdateAvailable(VersionDiff),

    /// Always contains `VersionDiff::Outdated`
    VoiceOutdated(VersionDiff),

    /// Always contains `VersionDiff::NotInstalled`
    VoiceNotInstalled(VersionDiff),

    // Always contains `VersionDiff::Diff`
    GameUpdateAvailable(VersionDiff),

    /// Always contains `VersionDiff::Outdated`
    GameOutdated(VersionDiff),

    /// Always contains `VersionDiff::NotInstalled`
    GameNotInstalled(VersionDiff)
}

impl Default for LauncherState {
    fn default() -> Self {
        Self::Launch
    }
}

impl LauncherState {
    pub fn get<T: Fn(&str)>(status: T) -> anyhow::Result<Self> {
        let config = config::get()?;

        // Check wine existence
        if let None = config.try_get_wine_executable() {
            return Ok(Self::WineNotInstalled);
        }

        // Check prefix existence
        if !WinePrefix::exists_in(&config.game.wine.prefix) {
            return Ok(Self::PrefixNotExists);
        }

        // Check game installation status
        status("Updating game info...");

        let game = Game::new(&config.game.path);
        let diff = game.try_get_diff()?;

        Ok(match diff {
            VersionDiff::Latest(_) | VersionDiff::Predownload { .. } => {
                status("Updating voice info...");

                let mut predownload_voice = Vec::new();

                for voice_package in &config.game.voices {
                    let mut voice_package = VoicePackage::with_locale(match VoiceLocale::from_str(voice_package) {
                        Some(locale) => locale,
                        None => return Err(anyhow::anyhow!("Incorrect voice locale \"{}\" specified in the config", voice_package))
                    })?;

                    status(format!("Updating voice info ({})...", voice_package.locale().to_name()).as_str());

                    // Replace voice package struct with the one constructed in the game's folder
                    // so it'll properly calculate its difference instead of saying "not installed"
                    if voice_package.is_installed_in(&config.game.path) {
                        voice_package = match VoicePackage::new(get_voice_package_path(&config.game.path, voice_package.locale())) {
                            Some(locale) => locale,
                            None => return Err(anyhow::anyhow!("Failed to load {} voice package", voice_package.locale().to_name()))
                        };
                    }

                    let diff = voice_package.try_get_diff()?;

                    match diff {
                        VersionDiff::Latest(_) => (),
                        VersionDiff::Predownload { .. } => predownload_voice.push(diff),

                        VersionDiff::Diff { .. } => return Ok(Self::VoiceUpdateAvailable(diff)),
                        VersionDiff::Outdated { .. } => return Ok(Self::VoiceOutdated(diff)),
                        VersionDiff::NotInstalled { .. } => return Ok(Self::VoiceNotInstalled(diff))
                    }
                }

                status("Updating patch info...");

                let patch = Patch::try_fetch(config.patch.servers.clone(), consts::PATCH_FETCHING_TIMEOUT)?;

                if patch.is_applied(&config.game.path)? {
                    if let VersionDiff::Predownload { .. } = diff {
                        Self::PredownloadAvailable {
                            game: diff,
                            voices: predownload_voice
                        }
                    }

                    else {
                        Self::Launch
                    }
                }

                else {
                    Self::PatchAvailable(patch)
                }
            },
            VersionDiff::Diff { .. } => Self::GameUpdateAvailable(diff),
            VersionDiff::Outdated { .. } => Self::GameOutdated(diff),
            VersionDiff::NotInstalled { .. } => Self::GameNotInstalled(diff)
        })
    }
}
