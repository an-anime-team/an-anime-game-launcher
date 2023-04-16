use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use anime_launcher_sdk::components::wine;

use crate::*;
use crate::i18n::*;
use crate::ui::components::*;
use super::{App, AppMsg};

pub fn download_wine(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>) {
    let mut config = Config::get().unwrap();

    match wine::get_downloaded(&CONFIG.components.path, &config.game.wine.builds) {
        Ok(downloaded) => {
            // Select downloaded version
            if !downloaded.is_empty() {
                config.game.wine.selected = Some(downloaded[0].versions[0].name.clone());

                Config::update(config);

                sender.input(AppMsg::UpdateLauncherState {
                    perform_on_download_needed: false,
                    apply_patch_if_needed: false,
                    show_status_page: true
                });
            }

            // Or download new one if none is available
            else {
                let latest = wine::Version::latest(&CONFIG.components.path).expect("Failed to get latest wine version");

                // Choose selected wine version or use latest available one
                let wine = match &config.game.wine.selected {
                    Some(version) => match wine::Version::find_in(&config.components.path, version) {
                        Ok(Some(version)) => version,
                        _ => latest
                    }

                    None => latest
                };

                // Download wine version
                match Installer::new(wine.uri) {
                    Ok(mut installer) => {
                        if let Some(temp_folder) = &config.launcher.temp {
                            installer.temp_folder = temp_folder.to_path_buf();
                        }

                        sender.input(AppMsg::SetDownloading(true));

                        std::thread::spawn(clone!(@strong sender => move || {
                            installer.install(&config.game.wine.builds, clone!(@strong sender => move |state| {
                                match &state {
                                    InstallerUpdate::DownloadingError(err) => {
                                        tracing::error!("Downloading failed: {err}");

                                        sender.input(AppMsg::Toast {
                                            title: tr("downloading-failed"),
                                            description: Some(err.to_string())
                                        });
                                    }

                                    InstallerUpdate::UnpackingError(err) => {
                                        tracing::error!("Unpacking failed: {err}");

                                        sender.input(AppMsg::Toast {
                                            title: tr("unpacking-failed"),
                                            description: Some(err.clone())
                                        });
                                    }

                                    _ => ()
                                }

                                #[allow(unused_must_use)] {
                                    progress_bar_input.send(ProgressBarMsg::UpdateFromState(DiffUpdate::InstallerUpdate(state)));
                                }
                            }));

                            config.game.wine.selected = Some(wine.name.clone());

                            Config::update(config);

                            sender.input(AppMsg::SetDownloading(false));
                            sender.input(AppMsg::UpdateLauncherState {
                                perform_on_download_needed: false,
                                apply_patch_if_needed: false,
                                show_status_page: true
                            });
                        }));
                    }

                    Err(err) => sender.input(AppMsg::Toast {
                        title: tr("wine-install-failed"),
                        description: Some(err.to_string())
                    })
                }
            }
        }

        Err(err) => sender.input(AppMsg::Toast {
            title: tr("downloaded-wine-list-failed"),
            description: Some(err.to_string())
        })
    }
}
