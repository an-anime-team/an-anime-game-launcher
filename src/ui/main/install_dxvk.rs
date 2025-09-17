use relm4::prelude::*;

use anime_launcher_sdk::wincompatlib::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;

use std::time::SystemTime;

use crate::*;

use super::{App, AppMsg};

pub fn install_dxvk(sender: ComponentSender<App>) {
    let config = Config::get().unwrap();

    match config.get_selected_wine() {
        Ok(Some(wine)) => {
            sender.input(AppMsg::DisableButtons(true));

            std::thread::spawn(move || {
                let wine = wine
                    .to_wine(config.components.path, Some(config.game.wine.builds.join(&wine.name)))
                    .with_prefix(config.game.wine.prefix)
                    .with_loader(WineLoader::Current);

                let dxvk_folder = use_latest_dxvk(config.game.dxvk.builds);

                if let Err(err) = Dxvk::install(&wine, dxvk_folder, InstallParams::default()) {
                    tracing::error!("Failed to install DXVK: {}", err);

                    sender.input(AppMsg::Toast {
                        title: tr!("dxvk-install-failed"),
                        description: Some(err.to_string()),
                    });
                }

                sender.input(AppMsg::DisableButtons(false));
                sender.input(AppMsg::UpdateLauncherState {
                    perform_on_download_needed: false,
                    show_status_page: true,
                });
            });
        }

        Ok(None) => {
            tracing::error!("Failed to get selected wine executable");

            sender.input(AppMsg::Toast {
                title: tr!("failed-get-selected-wine"),
                description: None,
            });
        }

        Err(err) => {
            tracing::error!("Failed to get selected wine executable: {err}");

            sender.input(AppMsg::Toast {
                title: tr!("failed-get-selected-wine"),
                description: Some(err.to_string()),
            });
        }
    }

}

pub fn use_latest_dxvk(builds_path: PathBuf) -> PathBuf {
    let mut latest: Option<(SystemTime, PathBuf)> = None;

    if let Ok(entries) = std::fs::read_dir(&builds_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let key_file = path.join("x64/dxgi.dll");
            
            if !key_file.exists() {
                continue;
            }

            if let Ok(metadata) = std::fs::metadata(&key_file) {
                if let Ok(modified) = metadata.modified() {
                    match &latest {
                        Some((latest_time, _)) if *latest_time >= modified => {}
                        _ => latest = Some((modified, path.clone())),
                    }
                }
            }
        }
    }

    if let Some((_, path)) = latest {
        path
    } else {
        builds_path
    }
}