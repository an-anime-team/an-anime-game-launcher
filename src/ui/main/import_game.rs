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

pub fn import_game(sender: relm4::ComponentSender<App>, path: PathBuf) {
    // reject xdg-document-portal fuse mounts, files appear present but vanish
    // when the portal closes, so the game would silently break
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    for p in [path.as_path(), canonical.as_path()] {
        let s = p.to_string_lossy();
        if s.starts_with("/run/user/") && s.contains("/doc/") {
            sender.input(AppMsg::Toast {
                title: i18n!("import-game-path-runtime"),
                description: None
            });
            return;
        }
    }

    let mut config = match Config::get() {
        Ok(c) => c,
        Err(err) => {
            sender.input(AppMsg::Toast {
                title: i18n!("import-game-error"),
                description: Some(err.to_string())
            });
            return;
        }
    };

    let edition = config.launcher.edition;
    let game = Game::new(&path, edition);

    if !game.is_installed() {
        sender.input(AppMsg::Toast {
            title: i18n!("import-game-invalid-path"),
            description: None
        });
        return;
    }

    // write .version if missing so the launcher can detect the version
    let version_path = path.join(".version");
    if !version_path.exists() {
        match game.get_version() {
            Ok(version) => {
                if let Err(err) = std::fs::write(&version_path, version.to_string()) {
                    tracing::warn!("Failed to write .version during import: {err}");
                }
            }
            Err(err) => tracing::warn!("Failed to detect version during import: {err}")
        }
    }

    match edition {
        GameEdition::Global => config.game.path.global = path,
        GameEdition::China => config.game.path.china = path
    }
    Config::update(config);

    sender.input(AppMsg::UpdateLauncherState {
        perform_on_download_needed: false,
        show_status_page: true
    });
}
