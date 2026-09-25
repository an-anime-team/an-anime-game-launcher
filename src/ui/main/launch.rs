use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};

use relm4::prelude::*;
use gtk::prelude::*;

use anime_launcher_sdk::genshin::config::schema::prelude::LauncherBehavior;

use crate::*;

use super::{App, AppMsg};

fn software_renderer_status(log: &str) -> Option<bool> {
    let is_software = |value: &str| {
        let value = value.to_ascii_lowercase();
        value.contains("llvmpipe") || value.contains("lavapipe")
    };

    let renderer = log.lines().find_map(|line| {
        let Some((field, value)) = line.split_once(':') else {
            return None;
        };

        field
            .trim()
            .eq_ignore_ascii_case("renderer")
            .then(|| is_software(value))
    });

    renderer.or_else(|| {
        let device_names: Vec<_> = log
            .lines()
            .filter_map(|line| {
                let (field, value) = line.split_once(':')?;
                field
                    .trim()
                    .eq_ignore_ascii_case("device name")
                    .then_some(value)
            })
            .collect();

        (!device_names.is_empty())
            .then(|| device_names.into_iter().any(is_software))
    })
}

fn software_renderer_log_paths(wine_prefix: &Path) -> Vec<PathBuf> {
    let users_path = wine_prefix.join("drive_c/users");
    let Ok(users) = fs::read_dir(users_path) else {
        return Vec::new();
    };

    users
        .filter_map(Result::ok)
        .flat_map(|user| {
            let user_path = user.path();
            [
                user_path.join("AppData/LocalLow/miHoYo/Genshin Impact/output_log.txt"),
                user_path.join("AppData/LocalLow/miHoYo/YuanShen/output_log.txt")
            ]
        })
        .collect()
}

fn uses_software_renderer(wine_prefix: &Path, launched_at: SystemTime) -> Option<bool> {
    software_renderer_log_paths(wine_prefix)
        .iter()
        .filter(|path| {
            fs::metadata(path)
                .and_then(|metadata| metadata.modified())
                .is_ok_and(|modified| modified >= launched_at)
        })
        .filter_map(|path| fs::read_to_string(path).ok())
        .find_map(|log| software_renderer_status(&log))
}

fn notify_software_renderer(sender: &ComponentSender<App>) {
    tracing::warn!(
        "Genshin is using a software renderer; if using NVIDIA with Flatpak, check that the NVIDIA GL runtime matches the host driver version"
    );

    sender.input(AppMsg::Toast {
        title: tr!("software-rendering-detected"),
        description: Some(tr!("software-rendering-detected-description"))
    });
}

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
        let wine_prefix = config.game.wine.prefix.clone();
        let launched_at = SystemTime::now();
        let stop_renderer_check = Arc::new(AtomicBool::new(false));
        let renderer_warning_shown = Arc::new(AtomicBool::new(false));

        let monitor_stop = Arc::clone(&stop_renderer_check);
        let monitor_warning_shown = Arc::clone(&renderer_warning_shown);
        let monitor_sender = sender.clone();
        let monitor_prefix = wine_prefix.clone();
        let monitor_launched_at = launched_at;

        thread::spawn(move || {
            while !monitor_stop.load(Ordering::Relaxed) {
                if let Some(is_software) =
                    uses_software_renderer(&monitor_prefix, monitor_launched_at)
                {
                    if is_software && !monitor_warning_shown.swap(true, Ordering::Relaxed) {
                        notify_software_renderer(&monitor_sender);
                    }

                    break;
                }

                thread::sleep(Duration::from_secs(1));
            }
        });

        let suggest_timeout_fix = match anime_launcher_sdk::genshin::game::run() {
            Ok(suggest) => suggest,
            Err(err) => {
                tracing::error!("Failed to launch game: {err}");

                sender.input(AppMsg::Toast {
                    title: tr!("game-launching-failed"),
                    description: Some(err.to_string())
                });

                false
            }
        };

        stop_renderer_check.store(true, Ordering::Relaxed);

        // A game that exits before the monitor's first poll still gets the diagnostic.
        if uses_software_renderer(&wine_prefix, launched_at) == Some(true)
            && !renderer_warning_shown.swap(true, Ordering::Relaxed)
        {
            notify_software_renderer(&sender);
        }

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

#[cfg(test)]
mod tests {
    use super::software_renderer_status;

    #[test]
    fn detects_unity_software_renderer_fields() {
        assert_eq!(
            software_renderer_status("Renderer: llvmpipe (LLVM 21.1.8, 256 bits)"),
            Some(true)
        );
        assert_eq!(
            software_renderer_status("Device name: lavapipe (LLVM 21.1.8)"),
            Some(true)
        );
        assert_eq!(software_renderer_status("renderer: LLVMPIPE"), Some(true));
    }

    #[test]
    fn does_not_flag_hardware_or_non_renderer_mentions() {
        assert_eq!(
            software_renderer_status("Renderer: NVIDIA GeForce RTX 3060"),
            Some(false)
        );
        assert_eq!(
            software_renderer_status("warn: Skipping CPU adapter: llvmpipe"),
            None
        );
        assert_eq!(
            software_renderer_status("Renderer: Unknown GPU"),
            Some(false)
        );
    }

    #[test]
    fn prefers_the_actual_unity_renderer_over_cpu_adapter_mentions() {
        assert_eq!(
            software_renderer_status(
                "Renderer: NVIDIA GeForce RTX 3060\nDevice name: llvmpipe"
            ),
            Some(false)
        );
    }
}
