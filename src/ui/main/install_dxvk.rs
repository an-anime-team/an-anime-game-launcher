use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use anime_launcher_sdk::wincompatlib::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;

use anime_launcher_sdk::components::dxvk::Version;

use crate::*;
use crate::ui::components::*;

use super::{App, AppMsg};

pub fn install_dxvk(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>) {
    let config = Config::get().unwrap();

    match config.get_selected_wine() {
        Ok(Some(wine_config)) => {
            sender.input(AppMsg::DisableButtons(true));

            std::thread::spawn(clone!(
                #[strong] sender,
                move || {
                    let components_path = config.components.path.clone();

                    let wine = wine_config
                        .to_wine(
                            components_path.clone(),
                            Some(config.game.wine.builds.join(&wine_config.name)),
                        )
                        .with_prefix(config.game.wine.prefix)
                        .with_loader(WineLoader::Current);

                    let latest = match Version::latest(components_path) {
                        Ok(version) => version,
                        Err(err) => {
                            tracing::error!("Failed to get latest DXVK version: {}", err);
                            sender.input(AppMsg::Toast {
                                title: tr!("dxvk-install-failed"),
                                description: Some(err.to_string()),
                            });
                            sender.input(AppMsg::DisableButtons(false));
                            return;
                        }
                    };

                    let dxvk_folder = config.game.dxvk.builds.join(&latest.name);

                    if !dxvk_folder.exists() {
                        match Installer::new(latest.uri) {
                            Ok(mut installer) => {
                                if let Some(temp_folder) = &config.launcher.temp {
                                    installer.temp_folder = temp_folder.to_path_buf();
                                }

                                sender.input(AppMsg::SetDownloading(true));

                                installer.install(&config.game.dxvk.builds, clone!(
                                    #[strong] sender,
                                    move |state| {
                                        match state {
                                            InstallerUpdate::DownloadingError(ref err) => {
                                                tracing::error!("DXVK download failed: {err}");
                                                sender.input(AppMsg::Toast {
                                                    title: tr!("dxvk-download-error"),
                                                    description: Some(err.to_string())
                                                });
                                            }
                                            InstallerUpdate::UnpackingError(ref err) => {
                                                tracing::error!("DXVK unpacking failed: {err}");
                                                sender.input(AppMsg::Toast {
                                                    title: tr!("dxvk-unpack-error"),
                                                    description: Some(err.clone())
                                                });
                                            }
                                            _ => {}
                                        }

                                        #[allow(unused_must_use)] {
                                            progress_bar_input.send(ProgressBarMsg::UpdateFromState(
                                                DiffUpdate::InstallerUpdate(state)
                                            ));
                                        }
                                    }
                                ));

                                sender.input(AppMsg::SetDownloading(false));
                                sender.input(AppMsg::UpdateLauncherState {
                                    perform_on_download_needed: false,
                                    show_status_page: true
                                });
                            }
                            Err(err) => {
                                tracing::error!("Failed to download DXVK: {}", err);
                                sender.input(AppMsg::Toast {
                                    title: tr!("dxvk-download-error"),
                                    description: Some(err.to_string())
                                });
                            }
                        }
                    }

                    if let Err(err) = Dxvk::install(&wine, dxvk_folder, InstallParams::default()) {
                        tracing::error!("Failed to install DXVK: {}", err);
                        sender.input(AppMsg::Toast {
                            title: tr!("dxvk-install-failed"),
                            description: Some(err.to_string()),
                        });
                    }

                    sender.input(AppMsg::DisableButtons(false));
                    sender.input(AppMsg::UpdateLauncherState {
                        perform_on_download_needed: false,
                        show_status_page: true,
                    });
                }
            ));
        }

        Ok(None) => {
            tracing::error!("Failed to get selected wine executable");
            sender.input(AppMsg::Toast {
                title: tr!("failed-get-selected-wine"),
                description: None,
            });
        }

        Err(err) => {
            tracing::error!("Failed to get selected wine executable: {err}");
            sender.input(AppMsg::Toast {
                title: tr!("failed-get-selected-wine"),
                description: Some(err.to_string()),
            });
        }
    }
}
