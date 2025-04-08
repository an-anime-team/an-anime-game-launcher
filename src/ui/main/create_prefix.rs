use relm4::prelude::*;

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
                        title: tr!("wine-prefix-update-failed"),
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
                title: tr!("failed-get-selected-wine"),
                description: None
            });
        }

        Err(err) => {
            tracing::error!("Failed to get selected wine executable: {err}");

            sender.input(AppMsg::Toast {
                title: tr!("failed-get-selected-wine"),
                description: Some(err.to_string())
            });
        }
    }
}
