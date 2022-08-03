use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

use gtk::glib;

use anime_game_core::prelude::*;

use crate::lib::prettify_bytes::prettify_bytes;

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
            InstallerUpdate::CheckingFreeSpace(_) => self.progress_bar.set_text(Some("Checking free space...")),
            InstallerUpdate::DownloadingStarted(_) => (),

            InstallerUpdate::DownloadingProgress(curr, total) => {
                let progress = curr as f64 / total as f64;

                self.update(progress, Some(&format!(
                    "Downloading: {:.2}% ({} of {})",
                    progress * 100.0,
                    prettify_bytes(curr),
                    prettify_bytes(total)
                )));
            }

            InstallerUpdate::UnpackingProgress(curr, total) => {
                let progress = curr as f64 / total as f64;

                self.update(progress, Some(&format!(
                    "Unpacking: {:.2}% ({} of {})",
                    progress * 100.0,
                    prettify_bytes(curr),
                    prettify_bytes(total)
                )));
            }

            InstallerUpdate::DownloadingFinished => (),
            InstallerUpdate::UnpackingStarted(_) => (),

            InstallerUpdate::DownloadingError(err) => return ProgressUpdateResult::Error(String::from("Failed to download"), err.into()),
            InstallerUpdate::UnpackingError => return ProgressUpdateResult::Error(String::from("Failed to unpack"), std::io::Error::last_os_error()),

            InstallerUpdate::UnpackingFinished => return ProgressUpdateResult::Finished,
        }

        ProgressUpdateResult::Updated
    }
}

unsafe impl Send for ProgressBar {}
unsafe impl Sync for ProgressBar {}
