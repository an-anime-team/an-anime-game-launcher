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

use std::path::PathBuf;

use crate::*;

use super::{App, AppMsg};

pub fn migrate_folder(sender: ComponentSender<App>, from: PathBuf, to: PathBuf, cleanup_folder: Option<PathBuf>) {
    sender.input(AppMsg::DisableButtons(true));

    std::thread::spawn(move || {
        move_files::move_files(&from, &to).expect("Failed to perform migration");

        if let Some(cleanup_folder) = cleanup_folder {
            std::fs::remove_dir_all(cleanup_folder).expect("Failed to remove cleanup folder");
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            show_status_page: true
        });
    });
}
