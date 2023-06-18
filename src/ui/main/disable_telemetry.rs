use std::process::Command;

use relm4::prelude::*;

use crate::*;
use crate::i18n::*;

use super::{App, AppMsg};

pub fn disable_telemetry(sender: ComponentSender<App>) {
    sender.input(AppMsg::DisableButtons(true));

    let config = Config::get().unwrap();

    std::thread::spawn(move || {
        let telemetry = config.launcher.edition
            .telemetry_servers()
            .iter()
            .map(|server| format!("0.0.0.0 {server}"))
            .collect::<Vec<String>>()
            .join("\\n");

        let output = if config.patch.root {
            Command::new("pkexec")
                .arg("echo")
                .arg("-e")
                .arg(format!("\\n{telemetry}\\n"))
                .arg(">>")
                .arg("/etc/hosts")
                .spawn()
        }

        else {
            Command::new("echo")
                .arg("-e")
                .arg(format!("\\n{telemetry}\\n"))
                .arg(">>")
                .arg("/etc/hosts")
                .spawn()
        };

        match output.and_then(|child| child.wait_with_output()) {
            Ok(output) => if !output.status.success() {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: tr("telemetry-servers-disabling-error"),
                    description: Some(String::from_utf8(output.stderr).unwrap())
                });
            }

            Err(err) => {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: tr("telemetry-servers-disabling-error"),
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
