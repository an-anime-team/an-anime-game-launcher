use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;

use std::io::Error;

use anime_game_core::prelude::*;
use wait_not_await::Await;

#[derive(Debug)]
pub enum ProgressUpdateResult {
    Updated,
    Error(String, std::io::Error),
    Finished
}

#[derive(Clone, glib::Downgrade)]
pub struct ProgressBar {
    pub progress_bar: gtk::ProgressBar,
    pub default_group: adw::PreferencesGroup,
    pub progress_bar_group: adw::PreferencesGroup
}

impl ProgressBar {
    pub fn new(progress_bar: gtk::ProgressBar, default_group: adw::PreferencesGroup, progress_bar_group: adw::PreferencesGroup) -> Self {
        Self {
            progress_bar,
            default_group,
            progress_bar_group
        }
    }

    pub fn show(&self) {
        self.progress_bar.set_text(None);
        self.progress_bar.set_fraction(0.0);

        self.default_group.hide();
        self.progress_bar_group.show();
    }

    pub fn hide(&self) {
        self.default_group.show();
        self.progress_bar_group.hide();
    }

    pub fn update(&self, fraction: f64, text: Option<&str>) {
        self.progress_bar.set_fraction(fraction);
        self.progress_bar.set_text(text);
    }

    pub fn update_from_state(&self, state: InstallerUpdate) -> ProgressUpdateResult {
        match state {
            InstallerUpdate::DownloadingStarted(_) => self.show(),

            InstallerUpdate::DownloadingProgress(curr, total) => {
                let progress = curr as f64 / total as f64;

                self.update(progress, None);
            }

            InstallerUpdate::UnpackingProgress(curr, total) => {
                let progress = curr as f64 / total as f64;

                self.update(progress, None);
            }

            InstallerUpdate::DownloadingFinished => (),
            InstallerUpdate::UnpackingStarted(_) => (),

            InstallerUpdate::DownloadingError(err) => return ProgressUpdateResult::Error(String::from("Failed to download"), err.into()),
            InstallerUpdate::UnpackingError => return ProgressUpdateResult::Error(String::from("Failed to unpack"), Error::last_os_error()),

            InstallerUpdate::UnpackingFinished => return ProgressUpdateResult::Finished
        }

        ProgressUpdateResult::Updated
    }
}

unsafe impl Send for ProgressBar {}
unsafe impl Sync for ProgressBar {}
