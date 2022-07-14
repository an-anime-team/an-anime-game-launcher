use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::Align;

use std::path::Path;

use anime_game_core::prelude::*;
use wait_not_await::Await;

use crate::lib::wine::Version;

#[derive(Debug)]
pub enum DownloadingResult {
    DownloadingError(std::io::Error),
    UnpackingError,
    Done
}

#[derive(Debug, Clone)]
pub struct WineRow {
    pub version: Version,

    pub row: adw::ActionRow,
    pub button: gtk::Button,
    pub progress_bar: gtk::ProgressBar
}

impl WineRow {
    pub fn new(version: Version) -> Self {
        let row = adw::ActionRow::new();
        let button = gtk::Button::new();

        row.set_title(&version.title);
        row.set_visible(version.recommended);

        button.set_icon_name("document-save-symbolic");
        button.set_valign(gtk::Align::Center);
        button.add_css_class("flat");

        row.add_suffix(&button);

        let progress_bar = gtk::ProgressBar::new();

        progress_bar.set_text(Some("Downloading: 0%"));
        progress_bar.set_show_text(true);

        progress_bar.set_width_request(200);
        progress_bar.set_valign(Align::Center);
        progress_bar.set_visible(false);

        row.add_suffix(&progress_bar);

        Self {
            version,
            row,
            button,
            progress_bar
        }
    }

    pub fn is_downloaded<T: ToString>(&self, runners_folder: T) -> bool {
        Path::new(&format!("{}/{}", runners_folder.to_string(), self.version.name)).exists()
    }

    pub fn update_state<T: ToString>(&self, runners_folder: T) {
        if self.is_downloaded(runners_folder) {
            self.button.set_icon_name("user-trash-symbolic");
        }

        else {
            self.button.set_icon_name("document-save-symbolic");
        }
    }

    /// Download wine
    /// 
    /// This method doesn't update components states, so you need to call `update_state` method manually
    pub fn download<T: ToString>(&self, runners_folder: T) -> Result<Await<DownloadingResult>, std::io::Error> {
        let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);
        let this = self.clone();

        this.progress_bar.set_visible(true);
        this.button.set_visible(false);

        let (downl_send, downl_recv) = std::sync::mpsc::channel();

        receiver.attach(None, move |state| {
            match state {
                InstallerUpdate::DownloadingStarted(_) => (),
                InstallerUpdate::DownloadingFinished => (),
                InstallerUpdate::UnpackingStarted(_) => (),

                InstallerUpdate::DownloadingProgress(curr, total) => {
                    let progress = curr as f64 / total as f64;

                    this.progress_bar.set_fraction(progress);
                    this.progress_bar.set_text(Some(&format!("Downloading: {}%", (progress * 100.0) as u64)));
                },

                InstallerUpdate::UnpackingProgress(curr, total) => {
                    let progress = curr as f64 / total as f64;

                    this.progress_bar.set_fraction(progress);
                    this.progress_bar.set_text(Some(&format!("Unpacking: {}%", (progress * 100.0) as u64)));
                },

                InstallerUpdate::UnpackingFinished => {
                    this.progress_bar.set_visible(false);
                    this.button.set_visible(true);

                    downl_send.send(DownloadingResult::Done);
                },

                InstallerUpdate::DownloadingError(err) => {
                    downl_send.send(DownloadingResult::DownloadingError(err.into()));
                },

                InstallerUpdate::UnpackingError => {
                    downl_send.send(DownloadingResult::UnpackingError);
                }
            }

            glib::Continue(true)
        });

        let (send, recv) = std::sync::mpsc::channel();

        let installer = Installer::new(&self.version.uri)?;
        let runners_folder = runners_folder.to_string();

        send.send(installer);

        std::thread::spawn(move || {
            let mut installer = recv.recv().unwrap();

            installer.install(runners_folder, move |state| {
                sender.send(state);
            });
        });

        Ok(Await::new(move || {
            downl_recv.recv().unwrap()
        }))
    }
}

unsafe impl Send for WineRow {}
unsafe impl Sync for WineRow {}
