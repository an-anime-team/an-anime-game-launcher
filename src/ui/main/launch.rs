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

use gtk::prelude::*;

use anime_launcher_sdk::genshin::config::schema::prelude::LauncherBehavior;

use crate::*;

use super::{App, AppMsg};

pub fn launch(sender: ComponentSender<App>) {
    let config = Config::get().unwrap();

    match config.launcher.behavior {
        // Disable launch button and show kill game button if behavior set to "Nothing" to prevent sussy actions
        LauncherBehavior::Nothing => {
            sender.input(AppMsg::DisableButtons(true));
            sender.input(AppMsg::SetKillGameButton(true));
        }

        // Hide launcher window if behavior set to "Hide" or "Close"
        LauncherBehavior::Hide | LauncherBehavior::Close => sender.input(AppMsg::HideWindow)
    }

    std::thread::spawn(move || {
        let suggest_timeout_fix = match anime_launcher_sdk::genshin::game::run() {
            Ok(suggest) => suggest,
            Err(err) => {
                tracing::error!("Failed to launch game: {err}");

                sender.input(AppMsg::Toast {
                    title: i18n!("game-launching-failed"),
                    description: Some(err.to_string())
                });

                false
            }
        };

        match config.launcher.behavior {
            // Enable launch button and hide kill game button if behavior set to "Nothing" after the game has closed
            LauncherBehavior::Nothing => {
                sender.input(AppMsg::DisableButtons(false));
                sender.input(AppMsg::SetKillGameButton(false));

                if suggest_timeout_fix {
                    sender.input(AppMsg::SuggestTimeoutFix);
                }
            }

            // Show back launcher window if behavior set to "Hide" and the game has closed
            LauncherBehavior::Hide => {
                sender.input(AppMsg::ShowWindow);

                if suggest_timeout_fix {
                    sender.input(AppMsg::SuggestTimeoutFix);
                }
            }

            // Otherwise close the launcher if behavior set to "Close" and the game has closed
            // We're calling quit method from the main context here because otherwise app won't be closed properly
            // (No timeout fix suggestion here since the app is quitting)
            LauncherBehavior::Close => gtk::glib::MainContext::default().invoke(|| {
                relm4::main_application().quit();
            })
        }
    });
}
