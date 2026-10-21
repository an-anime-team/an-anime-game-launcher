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

use anime_launcher_sdk::wincompatlib::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;

use crate::*;

use super::{App, AppMsg};

pub fn create_prefix(sender: ComponentSender<App>) {
    let config = Config::get().unwrap();

    match config.get_selected_wine() {
        Ok(Some(wine)) => {
            sender.input(AppMsg::DisableButtons(true));

            std::thread::spawn(move || {
                let wine = wine.to_wine(config.components.path, Some(config.game.wine.builds.join(&wine.name)))
                    .with_prefix(&config.game.wine.prefix)
                    .with_loader(WineLoader::Current);

                if let Err(err) = wine.init_prefix(None::<&str>) {
                    tracing::error!("Failed to create wine prefix");

                    sender.input(AppMsg::Toast {
                        title: i18n!("wine-prefix-update-failed"),
                        description: Some(err.to_string())
                    });
                }

                sender.input(AppMsg::DisableButtons(false));
                sender.input(AppMsg::UpdateLauncherState {
                    perform_on_download_needed: false,
                    show_status_page: true
                });
            });
        }

        Ok(None) => {
            tracing::error!("Failed to get selected wine executable");

            sender.input(AppMsg::Toast {
                title: i18n!("failed-get-selected-wine"),
                description: None
            });
        }

        Err(err) => {
            tracing::error!("Failed to get selected wine executable: {err}");

            sender.input(AppMsg::Toast {
                title: i18n!("failed-get-selected-wine"),
                description: Some(err.to_string())
            });
        }
    }
}
