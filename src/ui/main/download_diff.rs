use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use anime_launcher_sdk::config;
use anime_launcher_sdk::anime_game_core::installer::diff::VersionDiff;

use crate::*;
use crate::i18n::*;
use crate::ui::components::*;
use super::{App, AppMsg};

pub fn download_diff(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>, diff: VersionDiff) {
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
        let config = config::get().unwrap();

        #[allow(unused_must_use)]
        let result = diff.install_to_by(config.game.path, config.launcher.temp, clone!(@strong sender => move |state| {
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

            progress_bar_input.send(ProgressBarMsg::UpdateFromState(state));
        }));

        if let Err(err) = result {
            tracing::error!("Downloading failed: {err}");

            sender.input(AppMsg::Toast {
                title: tr("downloading-failed"),
                description: Some(err.to_string())
            });
        }

        sender.input(AppMsg::SetDownloading(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: true,
            apply_patch_if_needed: false,
            show_status_page: false
        });
    });
}
