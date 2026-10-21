// SPDX-License-Identifier: GPL-3.0-or-later
//
// an-anime-game-launcher
// Copyright (C) 2021 - 2026  Nikita Podvirnyi <krypt0nn@dawn.wine>
// Copyright (C) 2025 - 2026  @JohnTheCoolingFan <ivan8215145640@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use relm4::Sender;

use gtk::glib::clone;

use crate::*;
use crate::ui::components::*;
use super::{App, AppMsg};

pub fn download_diff(
    sender: ComponentSender<App>,
    progress_bar_input: Sender<ProgressBarMsg>,
    mut diff: VersionDiff
) {
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
        let config = Config::get().unwrap();
        let game_path = config
            .game
            .path
            .for_edition(config.launcher.edition)
            .to_path_buf();

        if !game_path.exists() && let Err(err) = std::fs::create_dir(&game_path) {
            tracing::error!(?err, "Failed to create game directory");

            sender.input(AppMsg::Toast {
                title: i18n!("downloading-failed"),
                description: Some(err.to_string())
            });
        }

        if let Some(temp) = config.launcher.temp {
            diff = diff.with_temp_folder(temp);
        }

        let result = diff.install_to(
            game_path,
            config.launcher.sophon.threads as usize,
            clone!(
                #[strong]
                sender,
                move |state| {
                    match &state {
                        DiffUpdate::InstallerUpdate(InstallerUpdate::DownloadingError(err)) => {
                            tracing::error!("Downloading failed: {err}");

                            sender.input(AppMsg::Toast {
                                title: i18n!("downloading-failed"),
                                description: Some(err.to_string())
                            });
                        }

                        DiffUpdate::InstallerUpdate(InstallerUpdate::UnpackingError(err)) => {
                            tracing::error!("Unpacking failed: {err}");

                            sender.input(AppMsg::Toast {
                                title: i18n!("unpacking-failed"),
                                description: Some(err.clone())
                            });
                        }

                        _ => ()
                    }

                    #[allow(unused_must_use)]
                    {
                        progress_bar_input.send(ProgressBarMsg::UpdateFromState(state));
                    }
                }
            )
        );

        let mut perform_on_download_needed = true;

        if let Err(err) = result {
            tracing::error!("Downloading failed: {err}");

            sender.input(AppMsg::Toast {
                title: i18n!("downloading-failed"),
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
