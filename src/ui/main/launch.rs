use relm4::prelude::*;

use crate::i18n::*;
use super::{App, AppMsg};

pub fn launch(sender: ComponentSender<App>) {
    sender.input(AppMsg::HideWindow);

    std::thread::spawn(move || {
        if let Err(err) = anime_launcher_sdk::genshin::game::run() {
            tracing::error!("Failed to launch game: {err}");

            sender.input(AppMsg::Toast {
                title: tr("game-launching-failed"),
                description: Some(err.to_string())
            });
        }

        sender.input(AppMsg::ShowWindow);
    });
}
