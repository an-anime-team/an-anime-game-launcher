use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::{Error, ErrorKind};

use anime_game_core::prelude::*;
use wait_not_await::Await;

use crate::ui::components::progress_bar::ProgressBar;
use crate::lib::config;
use crate::lib::prettify_bytes::prettify_bytes;

#[derive(Debug, Clone)]
pub enum LauncherState {
    Launch,
    PatchAvailable(Patch),

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
    pub fn get(status_page: Option<&adw::StatusPage>) -> std::io::Result<Self> {
        let config = config::get()?;
        let game = Game::new(&config.game.path);

        if let Some(status_page) = &status_page {
            status_page.set_description(Some("Updating game info..."));
        }

        let diff = game.try_get_diff()?;

        Ok(match diff {
            VersionDiff::Latest(_) => {
                if let Some(status_page) = &status_page {
                    status_page.set_description(Some("Updating voice info..."));
                }

                for voice_package in &config.game.voices {
                    let mut voice_package = VoicePackage::with_locale(match VoiceLocale::from_str(voice_package) {
                        Some(locale) => locale,
                        None => return Err(Error::new(ErrorKind::Other, format!("Incorrect voice locale \"{}\" specified in the config", voice_package)))
                    })?;

                    if let Some(status_page) = &status_page {
                        status_page.set_description(Some(format!("Updating voice info ({})...", voice_package.locale().to_name()).as_str()));
                    }

                    // Replace voice package struct with the one constructed in the game's folder
                    // so it'll properly calculate its difference instead of saying "not installed"
                    if voice_package.is_installed_in(&config.game.path) {
                        voice_package = match VoicePackage::new(get_voice_package_path(&config.game.path, voice_package.locale())) {
                            Some(locale) => locale,
                            None => return Err(Error::new(ErrorKind::Other, format!("Failed to load {} voice package", voice_package.locale().to_name())))
                        };
                    }

                    let diff = voice_package.try_get_diff()?;

                    match diff {
                        VersionDiff::Latest(_) => continue,
                        VersionDiff::Diff { .. } => return Ok(Self::VoiceUpdateAvailable(diff)),
                        VersionDiff::Outdated { .. } => return Ok(Self::VoiceOutdated(diff)),
                        VersionDiff::NotInstalled { .. } => return Ok(Self::VoiceNotInstalled(diff))
                    }
                }

                if let Some(status_page) = &status_page {
                    status_page.set_description(Some("Updating patch info..."));
                }

                let patch = Patch::try_fetch(config.patch.servers.clone())?;

                if patch.is_applied(&config.game.path)? {
                    Self::Launch
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

    /*pub fn execute(&self, progress_bar: &ProgressBar) -> Await<Result<(), (&str, Error)>> {
        match self {
            Self::Launch => {
                // Display toast message if the game is failed to run
                /*if let Err(err) = game::run(false) {
                    this.toast_error("Failed to run game", err);
                }*/

                todo!();
            },

            Self::PatchAvailable(_) => todo!(),

            Self::VoiceUpdateAvailable(diff) |
            Self::VoiceNotInstalled(diff) |
            Self::GameUpdateAvailable(diff) |
            Self::GameNotInstalled(diff) => {
                // this.update(Actions::DownloadDiff(Rc::new(diff))).unwrap();

                // Download wine version if not installed
                match WineVersion::latest() {
                    Ok(wine) => match Installer::new(wine.uri) {
                        Ok(mut installer) => {
                            let (send, recv) = std::sync::mpsc::channel();
                            let wine_title = wine.title.clone();

                            installer.install(&config.game.wine.builds, clone!(@strong this => move |state| {
                                match state {
                                    InstallerUpdate::UnpackingFinished => {
                                        send.send(true).unwrap();
                                    }

                                    InstallerUpdate::DownloadingError(_) |
                                    InstallerUpdate::UnpackingError => {
                                        send.send(false).unwrap();
                                    }

                                    _ => ()
                                }

                                this.update(Actions::UpdateProgressByState(Rc::new((state, Some(wine_title.clone()))))).unwrap();
                            }));

                            // Block thread until downloading finished
                            if recv.recv().unwrap() {
                                config.game.wine.selected = Some(wine.name);

                                config::update(config.clone());
                            }

                            else {
                                println!("I'm tired, Boss!");

                                return;
                            }
                        },
                        Err(err) => {
                            toast_error(&this, "Failed to init wine version installer", err.into());

                            return;
                        }
                    },
                    Err(err) => {
                        toast_error(&this, "Failed to load wine versions list", err.into());

                        return;
                    }
                }

                // Create prefix if needed
                let prefix = WinePrefix::new(&config.game.wine.prefix);

                if !prefix.exists() {
                    this.update(Actions::UpdateProgress {
                        fraction: Rc::new(0.0),
                        title: Rc::new(String::from("Creating prefix..."))
                    }).unwrap();

                    match config.try_get_selected_wine_info() {
                        Some(wine_version) => {
                            if let Err(err) = prefix.update(&config.game.wine.builds, wine_version) {
                                toast_error(&this, "Failed to create wineprefix", err);

                                return;
                            }
                        },
                        None => return
                    }
                }

                // Download and apply DXVK if not installed
                match DxvkVersion::latest() {
                    Ok(dxvk) => match Installer::new(&dxvk.uri) {
                        Ok(mut installer) => {
                            let (send, recv) = std::sync::mpsc::channel();
                            let dxvk_title = dxvk.name.clone();

                            installer.install(&config.game.dxvk.builds, clone!(@strong this => move |state| {
                                match state {
                                    InstallerUpdate::UnpackingFinished => {
                                        send.send(true).unwrap();
                                    }

                                    InstallerUpdate::DownloadingError(_) |
                                    InstallerUpdate::UnpackingError => {
                                        send.send(false).unwrap();
                                    }

                                    _ => ()
                                }

                                this.update(Actions::UpdateProgressByState(Rc::new((state, Some(dxvk_title.clone()))))).unwrap();
                            }));

                            // Block thread until downloading finished
                            if recv.recv().unwrap() {
                                config.game.dxvk.selected = Some(dxvk.name.clone());

                                config::update(config.clone());
                            }

                            else {
                                return;
                            }

                            // Apply DXVK
                            this.update(Actions::UpdateProgress {
                                fraction: Rc::new(100.0),
                                title: Rc::new(String::from("Applying DXVK..."))
                            }).unwrap();

                            match dxvk.apply(&config.game.dxvk.builds, &config.game.wine.prefix) {
                                Ok(_) => {
                                    config.game.dxvk.selected = Some(dxvk.name);
                                    
                                    config::update(config.clone());
                                },
                                Err(err) => {
                                    toast_error(&this, "Failed to apply DXVK", err);

                                    return;
                                }
                            }
                        },
                        Err(err) => {
                            toast_error(&this, "Failed to init wine version installer", err.into());

                            return;
                        }
                    },
                    Err(err) => {
                        toast_error(&this, "Failed to load wine versions list", err.into());

                        return;
                    }
                }

                todo!();
            },

            Self::GameOutdated(_) => (),
            Self::VoiceOutdated(_) => ()
        }

        todo!();
    }*/
}
