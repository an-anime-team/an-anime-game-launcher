use std::process::Command;
use std::path::PathBuf;

use relm4::prelude::*;

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
                    title: tr!("telemetry-servers-disabling-error"),
                    description: None // stdout/err is empty
                });
            }

            Err(err) => {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: tr!("telemetry-servers-disabling-error"),
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
