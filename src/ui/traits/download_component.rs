use gtk4::{self as gtk, prelude::*};

use gtk::glib;

use std::path::{Path, PathBuf};

use anime_game_core::prelude::*;
use wait_not_await::Await;

use crate::lib::config;

#[derive(Debug)]
pub enum DownloadingResult {
    DownloadingError(DownloadingError),
    UnpackingError(String),
    Done
}

pub trait DownloadComponent {
    fn get_component_path<T: ToString>(&self, installation_path: T) -> String;
    fn get_downloading_widgets(&self) -> (gtk::ProgressBar, gtk::Button);
    fn get_download_uri(&self) -> String;

    fn is_downloaded<T: ToString>(&self, installation_path: T) -> bool {
        Path::new(&self.get_component_path(installation_path)).exists()
    }

    fn download<T: ToString>(&self, installation_path: T) -> std::io::Result<Await<DownloadingResult>> {
        let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);
        let (progress_bar, button) = self.get_downloading_widgets();

        progress_bar.set_visible(true);
        button.set_visible(false);

        let (downl_send, downl_recv) = std::sync::mpsc::channel();

        receiver.attach(None, move |state| {
            match state {
                InstallerUpdate::DownloadingStarted(_) => (),
                InstallerUpdate::DownloadingFinished => (),
                InstallerUpdate::UnpackingStarted(_) => (),

                InstallerUpdate::CheckingFreeSpace(_) => {
                    progress_bar.set_text(Some("Checking free space..."));
                }

                InstallerUpdate::DownloadingProgress(curr, total) => {
                    let progress = curr as f64 / total as f64;

                    progress_bar.set_fraction(progress);
                    progress_bar.set_text(Some(&format!("Downloading: {}%", (progress * 100.0) as u64)));
                }

                InstallerUpdate::UnpackingProgress(curr, total) => {
                    let progress = curr as f64 / total as f64;

                    progress_bar.set_fraction(progress);
                    progress_bar.set_text(Some(&format!("Unpacking: {}%", (progress * 100.0) as u64)));
                }

                InstallerUpdate::UnpackingFinished => {
                    progress_bar.set_visible(false);
                    button.set_visible(true);

                    downl_send.send(DownloadingResult::Done).unwrap();
                }

                InstallerUpdate::DownloadingError(err) => {
                    downl_send.send(DownloadingResult::DownloadingError(err.into())).unwrap();
                }

                InstallerUpdate::UnpackingError(err) => {
                    downl_send.send(DownloadingResult::UnpackingError(err.to_string())).unwrap();
                }
            }

            glib::Continue(true)
        });

        let (send, recv) = std::sync::mpsc::channel();
        let config = config::get()?;

        let mut installer = Installer::new(self.get_download_uri())?;

        if let Some(temp_folder) = config.launcher.temp {
            installer.temp_folder = PathBuf::from(temp_folder);
        }

        installer.downloader
            .set_downloading_speed(config.launcher.speed_limit)
            .expect("Failed to set downloading speed limit");

        let installation_path = installation_path.to_string();

        send.send(installer).unwrap();

        std::thread::spawn(move || {
            let mut installer = recv.recv().unwrap();

            installer.install(installation_path, move |state| {
                sender.send(state).unwrap();
            });
        });

        Ok(Await::new(move || {
            downl_recv.recv().unwrap()
        }))
    }

    fn delete<T: ToString>(&self, installation_path: T) -> std::io::Result<()> {
        std::fs::remove_dir_all(self.get_component_path(installation_path))
    }
}
