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

use std::process::Command;
use std::path::PathBuf;

use crate::*;

use super::{App, AppMsg};

pub fn disable_telemetry(sender: ComponentSender<App>) {
    sender.input(AppMsg::DisableButtons(true));

    let config = Config::get().unwrap();

    std::thread::spawn(move || {
        let telemetry = config.launcher.edition
            .telemetry_servers()
            .iter()
            .map(|server| format!("echo '0.0.0.0 {server}' >> /etc/hosts"))
            .collect::<Vec<String>>()
            .join(" ; ");

        // TODO: perhaps find some another way? Or doesn't matter?
        let use_root = std::env::var("LAUNCHER_USE_ROOT")
            .map(|var| var == "1")
            .unwrap_or_else(|_| !PathBuf::from("/.flatpak-info").exists());

        let output = if use_root {
            Command::new("pkexec")
                .arg("bash")
                .arg("-c")
                .arg(format!("echo '' >> /etc/hosts ; {telemetry} ; echo '' >> /etc/hosts"))
                .spawn()
        }

        else {
            Command::new("bash")
                .arg("-c")
                .arg(format!("echo '' >> /etc/hosts ; {telemetry} ; echo '' >> /etc/hosts"))
                .spawn()
        };

        match output.and_then(|child| child.wait_with_output()) {
            Ok(output) => if !output.status.success() {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: i18n!("telemetry-servers-disabling-error"),
                    description: None // stdout/err is empty
                });
            }

            Err(err) => {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: i18n!("telemetry-servers-disabling-error"),
                    description: Some(err.to_string())
                });
            }
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            show_status_page: true
        });
    });
}
