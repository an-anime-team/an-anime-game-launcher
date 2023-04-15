use relm4::prelude::*;

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

            let config = Config::get().unwrap();

            std::thread::spawn(move || {
                let mut apply_patch_if_needed = true;

                if let Err(err) = patch.apply(config.game.path.for_edition(config.launcher.edition), config.patch.root) {
                    tracing::error!("Failed to patch the game");

                    sender.input(AppMsg::Toast {
                        title: tr("game-patching-error"),
                        description: Some(err.to_string())
                    });

                    // Don't try to apply the patch after state updating
                    // because we just failed to do it
                    apply_patch_if_needed = false;
                }

                sender.input(AppMsg::DisableButtons(false));
                sender.input(AppMsg::UpdateLauncherState {
                    perform_on_download_needed: false,
                    apply_patch_if_needed,
                    show_status_page: true
                });
            });
        }
    }
}
