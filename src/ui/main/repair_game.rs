use anime_launcher_sdk::anime_game_core::sophon::{self, repairer::{SophonRepairer, Update as SophonRepairerUpdate}};
use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use crate::*;
use crate::ui::components::*;

use super::{App, AppMsg};

#[allow(unused_must_use)]
pub fn repair_game(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>) {
    let config = Config::get().unwrap();

    progress_bar_input.send(ProgressBarMsg::UpdateCaption(Some(tr!("verifying-files"))));
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
        let client = Default::default();

        let game_branches_info = sophon::get_game_branches_info(Clone::clone(&client), config.launcher.edition.into()).unwrap();
        let game_branch_info = game_branches_info.get_game_latest_by_id(config.launcher.edition.game_id()).unwrap();

        let downloads = sophon::installer::get_game_download_sophon_info(client.clone(), &game_branch_info.main, config.launcher.edition.into()).unwrap();
        let game_download_info = downloads.manifests.iter().find(|sdi| sdi.matching_field == "game").unwrap();

        let mut manifests = vec![game_download_info];

        let game_path = config.game.path.for_edition(config.launcher.edition).to_path_buf();
        let game = Game::new(&game_path, config.launcher.edition);

        if let Ok(voiceovers) = game.get_voice_packages() {
            for package in voiceovers {
                if let Some(voice_download_info) = downloads.manifests.iter().find(|sdi| sdi.matching_field == package.locale().to_code()) {
                    manifests.push(voice_download_info);
                }
            }
        }

        let repairer = SophonRepairer::new(&manifests, client, config.launcher.temp.unwrap_or_else(std::env::temp_dir)).unwrap();

        let updater = move |msg: SophonRepairerUpdate| {
            match msg {
                SophonRepairerUpdate::VerifyingProgress { total, checked } => {
                    tracing::trace!("Verification progress [{checked}/{total}]");
                    progress_bar_input.send(ProgressBarMsg::UpdateProgressCounter(checked, total));
                }
                SophonRepairerUpdate::RepairingProgress { total, repaired } => {
                    tracing::trace!("Repairing progress [{repaired}/{total}]");
                    progress_bar_input.send(ProgressBarMsg::UpdateProgressCounter(repaired, total));
                }

                SophonRepairerUpdate::VerifyingStarted => {
                    tracing::trace!("Verification started");
                }
                SophonRepairerUpdate::VerifyingFinished { broken } => {
                    tracing::info!("Verification finished with {broken} broken files")
                }

                SophonRepairerUpdate::RepairingStarted => {
                    progress_bar_input.send(ProgressBarMsg::UpdateCaption(Some(tr!("repairing-files"))));
                    tracing::trace!("Repairing started");
                }
                SophonRepairerUpdate::RepairingFinished => {
                    tracing::trace!("Repair finished");
                }

                SophonRepairerUpdate::DownloadingError(err) => {
                    tracing::error!("Error during repairing: {err}")
                }
                SophonRepairerUpdate::FileHashCheckFailed(path) => {
                    tracing::error!("File hash check error for `{path:?}`")
                }
            }
        };

        repairer.check_and_repair(&game_path, config.launcher.repairer.threads as usize, updater);

        let _ = std::fs::remove_dir_all(repairer.downloading_temp());

        /*
        match repairer::try_get_integrity_files(config.launcher.edition, None) {
            Ok(mut files) => {
                // Add voiceovers files
                let game_path = config.game.path.for_edition(config.launcher.edition).to_path_buf();
                let game = Game::new(&game_path, config.launcher.edition);

                if let Ok(voiceovers) = game.get_voice_packages() {
                    for package in voiceovers {
                        if let Ok(mut voiceover_files) = repairer::try_get_voice_integrity_files(config.launcher.edition, package.locale(), None) {
                            files.append(&mut voiceover_files);
                        }
                    }
                }

                progress_bar_input.send(ProgressBarMsg::UpdateProgress(0, 0));

                let mut total = 0;

                for file in &files {
                    total += file.size;
                }

                let median_size = total / config.launcher.repairer.threads;
                let mut i = 0;

                let (verify_sender, verify_receiver) = std::sync::mpsc::channel();

                for _ in 0..config.launcher.repairer.threads {
                    let mut thread_files = Vec::new();
                    let mut thread_files_size = 0;

                    while i < files.len() {
                        thread_files.push(files[i].clone());

                        thread_files_size += files[i].size;
                        i += 1;

                        if thread_files_size >= median_size {
                            break;
                        }
                    }

                    let thread_sender = verify_sender.clone();

                    std::thread::spawn(clone!(
                        #[strong]
                        game_path, 

                        move || {
                            for file in thread_files {
                                let status = if config.launcher.repairer.fast {
                                    file.fast_verify(&game_path)
                                } else {
                                    file.verify(&game_path)
                                };

                                thread_sender.send((file, status)).unwrap();
                            }
                        }
                    ));
                }

                // We have [config.launcher.repairer.threads] copies of this sender + the original one
                // receiver will return Err when all the senders will be dropped.
                // [config.launcher.repairer.threads] senders will be dropped when threads will finish verifying files
                // but this one will live as long as current thread exists so we should drop it manually
                drop(verify_sender);

                let mut broken = Vec::new();
                let mut processed = 0;

                while let Ok((file, status)) = verify_receiver.recv() {
                    processed += file.size;

                    if !status {
                        broken.push(file);
                    }

                    progress_bar_input.send(ProgressBarMsg::UpdateProgress(processed, total));
                }

                if !broken.is_empty() {
                    let total = broken.len() as u64;

                    progress_bar_input.send(ProgressBarMsg::UpdateCaption(Some(tr!("repairing-files"))));
                    progress_bar_input.send(ProgressBarMsg::DisplayFraction(false));
                    progress_bar_input.send(ProgressBarMsg::UpdateProgress(0, total));

                    tracing::warn!("Found broken files:\n{}", broken.iter().fold(String::new(), |acc, file| acc + &format!("- {}\n", file.path.to_string_lossy())));

                    for (i, file) in broken.into_iter().enumerate() {
                        tracing::debug!("Repairing file: {}", file.path.to_string_lossy());

                        if let Err(err) = file.repair(&game_path) {
                            sender.input(AppMsg::Toast {
                                title: tr!("game-file-repairing-error"),
                                description: Some(err.to_string())
                            });

                            tracing::error!("Failed to repair game file: {err}");
                        }

                        progress_bar_input.send(ProgressBarMsg::UpdateProgress(i as u64 + 1, total));
                    }

                    progress_bar_input.send(ProgressBarMsg::DisplayFraction(true));
                }
            }

            Err(err) => {
                tracing::error!("Failed to get inregrity failes: {err}");

                sender.input(AppMsg::Toast {
                    title: tr!("integrity-files-getting-error"),
                    description: Some(err.to_string())
                });
            }
        }
        */

        sender.input(AppMsg::SetDownloading(false));
    });
}
