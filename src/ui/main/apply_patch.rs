use relm4::prelude::*;

use anime_launcher_sdk::config;

use crate::*;
use crate::i18n::*;
use super::{App, AppMsg};

pub fn apply_patch<T: PatchExt + Send + Sync + 'static>(sender: ComponentSender<App>, patch: T) {
    match patch.status() {
        PatchStatus::NotAvailable |
        PatchStatus::Outdated { .. } |
        PatchStatus::Preparation { .. } => unreachable!(),

        PatchStatus::Testing { .. } |
        PatchStatus::Available { .. } => {
            sender.input(AppMsg::DisableButtons(true));

            let config = config::get().unwrap();

            std::thread::spawn(move || {
                if let Err(err) = patch.apply(&config.game.path, config.patch.root) {
                    tracing::error!("Failed to patch the game");

                    sender.input(AppMsg::Toast {
                        title: tr("game-patching-error"),
                        description: Some(err.to_string())
                    });
                }

                sender.input(AppMsg::DisableButtons(false));
                sender.input(AppMsg::UpdateLauncherState {
                    perform_on_download_needed: false,
                    apply_patch_if_needed: true,
                    show_status_page: true
                });
            });
        }
    }
}
