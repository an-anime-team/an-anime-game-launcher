use relm4::prelude::*;

use std::path::PathBuf;

use crate::*;
use super::{App, AppMsg};

pub fn migrate_folder(sender: ComponentSender<App>, from: PathBuf, to: PathBuf, cleanup_folder: Option<PathBuf>) {
    sender.input(AppMsg::DisableButtons(true));

    std::thread::spawn(move || {
        move_folder::move_folder(&from, &to).expect("Failed to perform migration");

        if let Some(cleanup_folder) = cleanup_folder {
            std::fs::remove_dir_all(cleanup_folder).expect("Failed to remove cleanup folder");
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            apply_patch_if_needed: false,
            show_status_page: true
        });
    });
}
