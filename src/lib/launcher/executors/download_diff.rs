use std::io::Error;

use anime_game_core::prelude::*;
use wait_not_await::Await;

use crate::ui::components::progress_bar::ProgressBar;
use crate::lib::prettify_bytes::prettify_bytes;

/*pub fn download_diff(diff: &VersionDiff, progress_bar: ProgressBar, suffix: Option<String>) -> Await<Result<(), (String, Error)>> {
    let (send, recv) = std::sync::mpsc::channel();

    diff.install(move |state| {
        match state {
            InstallerUpdate::DownloadingStarted(_) => progress_bar.update(0.0, Some("Downloading...")),

            InstallerUpdate::DownloadingProgress(curr, total) => {
                // To reduce amount of action requests
                // if curr % 10000 < 200 {
                    let progress = curr as f64 / total as f64;

                    progress_bar.update(progress, Some(&format!(
                        "Downloading{}: {:.2}% ({} of {})",
                        if let Some(suffix) = suffix { format!(" {}", suffix) } else { String::new() },
                        progress * 100.0,
                        prettify_bytes(curr),
                        prettify_bytes(total)
                    )));
                // }
            }

            InstallerUpdate::UnpackingStarted(_) => progress_bar.update(0.0, Some("Unpacking...")),

            InstallerUpdate::UnpackingProgress(curr, total) => {
                let progress = curr as f64 / total as f64;

                progress_bar.update(progress, Some(&format!(
                    "Unpacking{}: {:.2}% ({} of {})",
                    if let Some(suffix) = suffix { format!(" {}", suffix) } else { String::new() },
                    progress * 100.0,
                    prettify_bytes(curr),
                    prettify_bytes(total)
                )));
            }

            InstallerUpdate::DownloadingFinished => (),

            InstallerUpdate::UnpackingFinished => {
                send.send(Ok(()));
            },

            InstallerUpdate::DownloadingError(err) => {
                send.send(Err((String::from("Failed to download"), err.into())));
            }

            InstallerUpdate::UnpackingError => {
                send.send(Err((String::from("Failed to unpack"), Error::last_os_error())));
            }
        }
    });

    Await::new(move || {
        recv.recv().unwrap()
    })
}*/
