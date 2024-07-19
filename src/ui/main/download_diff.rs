use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use crate::*;
use crate::ui::components::*;

use super::{App, AppMsg};

pub fn download_diff(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>, mut diff: VersionDiff) {
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
        let config = Config::get().unwrap();
        let game_path = config.game.path.for_edition(config.launcher.edition).to_path_buf();

        if let Some(temp) = config.launcher.temp {
            diff = diff.with_temp_folder(temp);
        }

        let result = diff.install_to(game_path, clone!(
            #[strong]
            sender,

            move |state| {
                match &state {
                    DiffUpdate::InstallerUpdate(InstallerUpdate::DownloadingError(err)) => {
                        tracing::error!("Downloading failed: {err}");

                        sender.input(AppMsg::Toast {
                            title: tr!("downloading-failed"),
                            description: Some(err.to_string())
                        });
                    }

                    DiffUpdate::InstallerUpdate(InstallerUpdate::UnpackingError(err)) => {
                        tracing::error!("Unpacking failed: {err}");

                        sender.input(AppMsg::Toast {
                            title: tr!("unpacking-failed"),
                            description: Some(err.clone())
                        });
                    }

                    _ => ()
                }

                #[allow(unused_must_use)] {
                    progress_bar_input.send(ProgressBarMsg::UpdateFromState(state));
                }
            }
        ));

        let mut perform_on_download_needed = true;

        if let Err(err) = result {
            tracing::error!("Downloading failed: {err}");

            sender.input(AppMsg::Toast {
                title: tr!("downloading-failed"),
                description: Some(err.to_string())
            });

            // Don't try to download something after state updating
            // because we just failed to do it
            perform_on_download_needed = false;
        }

        sender.input(AppMsg::SetDownloading(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed,
            show_status_page: false
        });
    });
}
