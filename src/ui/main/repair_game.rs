use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use std::path::Path;

use crate::*;
use crate::i18n::*;
use crate::ui::components::*;
use super::{App, AppMsg};

#[allow(unused_must_use)]
pub fn repair_game(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>) {
    let config = Config::get().unwrap();

    progress_bar_input.send(ProgressBarMsg::UpdateCaption(Some(tr("verifying-files"))));
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
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

                    std::thread::spawn(clone!(@strong game_path => move || {
                        for file in thread_files {
                            let status = if config.launcher.repairer.fast {
                                file.fast_verify(&game_path)
                            } else {
                                file.verify(&game_path)
                            };

                            thread_sender.send((file, status)).unwrap();
                        }
                    }));
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
                    progress_bar_input.send(ProgressBarMsg::UpdateCaption(Some(tr("repairing-files"))));
                    progress_bar_input.send(ProgressBarMsg::DisplayFraction(false));
                    progress_bar_input.send(ProgressBarMsg::UpdateProgress(0, 0));

                    tracing::warn!("Found broken files:\n{}", broken.iter().fold(String::new(), |acc, file| acc + &format!("- {}\n", file.path.to_string_lossy())));

                    let total = broken.len() as f64;

                    // Get main patch status

                    let player_patch = UnityPlayerPatch::from_folder(&config.patch.path, config.launcher.edition)
                        .and_then(|patch| patch.is_applied(&game_path))
                        .unwrap_or_else(|err| {
                            tracing::warn!("Failed to get player patch status: {err}. Used config value instead: {}", config.patch.apply_main);

                            config.patch.apply_main
                        });

                    // Get xlua patch status

                    let xlua_patch = XluaPatch::from_folder(&config.patch.path, config.launcher.edition)
                        .and_then(|patch| patch.is_applied(&game_path))
                        .unwrap_or_else(|err| {
                            tracing::warn!("Failed to get xlua patch status: {err}. Used config value instead: {}", config.patch.apply_xlua);

                            config.patch.apply_xlua
                        });

                    tracing::debug!("Patches status: player({player_patch}), xlua({xlua_patch})");

                    fn should_ignore(path: &Path, player_patch: bool, xlua_patch: bool) -> bool {
                        // Files managed by launch.bat file
                        for part in ["crashreport.exe", "upload_crash.exe"] {
                            if path.ends_with(part) {
                                return true;
                            }
                        }

                        // UnityPlayer patch related files
                        if player_patch {
                            for part in ["UnityPlayer.dll", "vulkan-1.dll"] {
                                if path.ends_with(part) {
                                    return true;
                                }
                            }
                        }

                        // Xlua patch related files
                        if xlua_patch {
                            for part in ["xlua.dll", "mhypbase.dll"] {
                                if path.ends_with(part) {
                                    return true;
                                }
                            }
                        }

                        false
                    }

                    for (i, file) in broken.into_iter().enumerate() {
                        if !should_ignore(&file.path, player_patch, xlua_patch) {
                            tracing::debug!("Repairing file: {}", file.path.to_string_lossy());

                            if let Err(err) = file.repair(&game_path) {
                                sender.input(AppMsg::Toast {
                                    title: tr("game-file-repairing-error"),
                                    description: Some(err.to_string())
                                });

                                tracing::error!("Failed to repair game file: {err}");
                            }
                        }

                        else {
                            tracing::debug!("Skipped file: {}", file.path.to_string_lossy());
                        }

                        progress_bar_input.send(ProgressBarMsg::UpdateProgress(i as u64, total as u64));
                    }

                    progress_bar_input.send(ProgressBarMsg::DisplayFraction(true));
                }
            }

            Err(err) => {
                tracing::error!("Failed to get inregrity failes: {err}");

                sender.input(AppMsg::Toast {
                    title: tr("integrity-files-getting-error"),
                    description: Some(err.to_string())
                });
            }
        }

        sender.input(AppMsg::SetDownloading(false));
    });
}
