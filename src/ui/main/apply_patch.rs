use relm4::prelude::*;

use crate::*;

use super::{App, AppMsg};

pub fn apply_patch(sender: ComponentSender<App>, patch: PlayerPatch, rename_mhypbase: bool) {
    match patch.status() {
        PatchStatus::NotAvailable |
        PatchStatus::Outdated { .. } |
        PatchStatus::Preparation { .. } => unreachable!(),

        PatchStatus::Testing { .. } |
        PatchStatus::Available { .. } => {
            sender.input(AppMsg::DisableButtons(true));

            let config = Config::get().unwrap();

            std::thread::spawn(move || {
                if let Err(err) = patch.apply(config.game.path.for_edition(config.launcher.edition), config.patch.root) {
                    tracing::error!("Failed to patch the game");

                    sender.input(AppMsg::Toast {
                        title: tr!("game-patching-error"),
                        description: Some(err.to_string())
                    });
                }

                else if rename_mhypbase {
                    let game_folder = config.game.path.for_edition(patch.edition);

                    let mhypbase = game_folder.join("mhypbase.dll");
                    let mhypbase_bak = game_folder.join("mhypbase.dll.bak");

                    if mhypbase.exists() {
                        if let Err(err) = std::fs::rename(mhypbase, mhypbase_bak) {
                            tracing::error!("Failed to rename mhypbase file");

                            sender.input(AppMsg::Toast {
                                title: tr!("game-patching-error"),
                                description: Some(err.to_string())
                            });
                        }
                    }
                }

                sender.input(AppMsg::DisableButtons(false));
                sender.input(AppMsg::UpdateLauncherState {
                    perform_on_download_needed: false,
                    show_status_page: true
                });
            });
        }
    }
}
