// SPDX-License-Identifier: GPL-3.0-or-later
//
// an-anime-game-launcher
// Copyright (C) 2021 - 2026  Nikita Podvirnyi <krypt0nn@dawn.wine>
// Copyright (C) 2025 - 2026  @JohnTheCoolingFan <ivan8215145640@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use adw::prelude::*;

use anime_launcher_sdk::anime_game_core::sophon::installer::Update as SophonInstallerUpdate;
use anime_launcher_sdk::anime_game_core::sophon::updater::Update as SophonPatcherUpdate;

use crate::*;

pub struct ProgressBarInit {
    pub caption: Option<String>,

    /// Add progress percentage (`XX.YY%`) suffix
    pub display_progress: bool,

    /// Add `(XX MB of YY MB)` suffix
    pub display_fraction: bool,

    pub visible: bool
}

pub struct ProgressBar {
    pub fraction: f64,
    pub caption: Option<String>,

    /// e.g. (53.21 MB, 10 GB)
    pub downloaded: Option<(String, String)>,

    /// Add progress percentage (`XX.YY%`) suffix
    pub display_progress: bool,

    /// Add `(XX MB of YY MB)` suffix
    pub display_fraction: bool,

    pub visible: bool
}

#[derive(Debug)]
pub enum ProgressBarMsg {
    Reset,
    UpdateCaption(Option<String>),
    DisplayProgress(bool),
    DisplayFraction(bool),

    /// (current bytes, total bytes)
    UpdateProgress(u64, u64),

    /// (current items, total items)
    UpdateProgressCounter(u64, u64),

    UpdateFromState(DiffUpdate),
    SetVisible(bool)
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for ProgressBar {
    type Init = ProgressBarInit;
    type Input = ProgressBarMsg;
    type Output = ();

    view! {
        #[root]
        gtk::ProgressBar {
            set_valign: gtk::Align::Center,

            #[watch]
            set_visible: model.visible,

            #[watch]
            set_fraction: model.fraction,

            #[watch]
            set_show_text: model.caption.is_some(),

            #[watch]
            set_text: Some(&match model.caption.clone() {
                Some(mut caption) => {
                    if model.display_progress {
                        caption = format!("{caption}: {:.2}%", model.fraction * 100.0);
                    }

                    if model.display_fraction && let Some((curr, total)) = &model.downloaded {
                        caption = format!("{caption} ({curr} of {total})");
                    }

                    caption
                },
                None => String::new()
            })
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        let model = ProgressBar {
            fraction: 0.0,
            caption: init.caption,
            downloaded: None,
            display_progress: init.display_progress,
            display_fraction: init.display_fraction,
            visible: init.visible
        };

        let widgets = view_output!();

        AsyncComponentParts {
            model,
            widgets
        }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            ProgressBarMsg::Reset => {
                self.fraction = 0.0;
                self.downloaded = None;
                self.caption = None;
            }

            ProgressBarMsg::UpdateCaption(caption) => self.caption = caption,
            ProgressBarMsg::DisplayProgress(value) => self.display_progress = value,
            ProgressBarMsg::DisplayFraction(value) => self.display_fraction = value,

            ProgressBarMsg::UpdateProgress(curr, total) => {
                self.fraction = curr as f64 / total as f64;

                self.downloaded = Some((prettify_bytes(curr), prettify_bytes(total)));
            }

            ProgressBarMsg::UpdateProgressCounter(curr, total) => {
                self.fraction = curr as f64 / total as f64;

                self.downloaded = Some((curr.to_string(), total.to_string()));
            }

            ProgressBarMsg::UpdateFromState(state) => {
                match state {
                    // checking free space
                    DiffUpdate::CheckingFreeSpace(_)
                    | DiffUpdate::InstallerUpdate(InstallerUpdate::CheckingFreeSpace(_))
                    | DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::CheckingFreeSpace(_))
                    | DiffUpdate::SophonInstallerUpdate(
                        SophonInstallerUpdate::CheckingFreeSpace(_)
                    ) => self.caption = Some(i18n!("checking-free-space")),

                    // checking files before download/update
                    DiffUpdate::SophonInstallerUpdate(SophonInstallerUpdate::CheckingFiles {
                        ..
                    })
                    | DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::CheckingFilesStarted) => {
                        self.caption = Some(i18n!("verifying-files"));
                        self.display_fraction = false;
                    }

                    DiffUpdate::SophonInstallerUpdate(
                        SophonInstallerUpdate::CheckingFilesProgress {
                            passed,
                            total
                        }
                    ) => {
                        self.fraction = passed as f64 / total as f64;
                    }

                    // download started
                    DiffUpdate::InstallerUpdate(InstallerUpdate::DownloadingStarted(_))
                    | DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::DownloadingStarted(_))
                    | DiffUpdate::SophonInstallerUpdate(
                        SophonInstallerUpdate::DownloadingStarted {
                            ..
                        }
                    ) => {
                        self.caption = Some(i18n!("downloading"));
                        self.display_fraction = true;
                    }

                    DiffUpdate::InstallerUpdate(InstallerUpdate::UpdatingPermissionsStarted(_)) => {
                        self.caption = Some(i18n!("updating-permissions"))
                    }
                    DiffUpdate::InstallerUpdate(InstallerUpdate::UnpackingStarted(_)) => {
                        self.caption = Some(i18n!("unpacking"))
                    }

                    DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::DeletingStarted) => {
                        // messes with the progress bar label
                        // self.caption = Some(i18n!("removing-outdated"))
                    }
                    // not emitted by the core
                    DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::PatchingStarted) => {
                        self.caption = Some(i18n!("applying-hdiff"))
                    }

                    DiffUpdate::ApplyingHdiffStarted => {
                        self.caption = Some(i18n!("applying-hdiff"));

                        self.display_fraction = false;
                    }

                    DiffUpdate::RemovingOutdatedStarted => {
                        self.caption = Some(i18n!("removing-outdated"));

                        self.display_fraction = false;
                    }

                    DiffUpdate::InstallerUpdate(InstallerUpdate::DownloadingProgress(
                        curr,
                        total
                    ))
                    | DiffUpdate::InstallerUpdate(InstallerUpdate::UpdatingPermissions(
                        curr,
                        total
                    ))
                    | DiffUpdate::InstallerUpdate(InstallerUpdate::UnpackingProgress(
                        curr,
                        total
                    ))
                    | DiffUpdate::ApplyingHdiffProgress(curr, total)
                    | DiffUpdate::RemovingOutdatedProgress(curr, total) => {
                        self.fraction = curr as f64 / total as f64;

                        self.downloaded = Some((prettify_bytes(curr), prettify_bytes(total)));
                    }

                    DiffUpdate::InstallerUpdate(InstallerUpdate::UpdatingPermissionsFinished) => {
                        tracing::info!("Updating permissions finished")
                    }
                    DiffUpdate::InstallerUpdate(InstallerUpdate::UnpackingFinished) => {
                        tracing::info!("Unpacking finished")
                    }

                    DiffUpdate::InstallerUpdate(InstallerUpdate::DownloadingError(err)) => {
                        tracing::error!("Downloading error: {:?}", err)
                    }
                    DiffUpdate::InstallerUpdate(InstallerUpdate::UnpackingError(err)) => {
                        tracing::error!("Unpacking error: {:?}", err)
                    }
                    DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::PatchingError(err)) => {
                        tracing::error!("Patching error: {err:?}")
                    }

                    // applying hdiff finish
                    DiffUpdate::ApplyingHdiffFinished
                    | DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::PatchingFinished) => {
                        tracing::info!("Applying hdiffs finished")
                    }
                    // removing outdated finish
                    DiffUpdate::RemovingOutdatedFinished
                    | DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::DeletingFinished) => {
                        tracing::info!("Removing outdated files finished")
                    }
                    // downloading finish
                    DiffUpdate::InstallerUpdate(InstallerUpdate::DownloadingFinished)
                    | DiffUpdate::SophonInstallerUpdate(
                        SophonInstallerUpdate::DownloadingFinished
                    )
                    | DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::DownloadingFinished) => {
                        tracing::info!("Downloading finished")
                    }

                    // downloading errors
                    DiffUpdate::SophonInstallerUpdate(SophonInstallerUpdate::DownloadingError(
                        err
                    ))
                    | DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::DownloadingError(err)) =>
                    {
                        tracing::error!("Downloading error: {err:?}")
                    }
                    // file hash check errors
                    DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::FileHashCheckFailed(
                        path
                    )) => tracing::error!("File hash check failed on {path:?}"),

                    // sophon download progress reports
                    DiffUpdate::SophonInstallerUpdate(
                        SophonInstallerUpdate::DownloadingProgressBytes {
                            downloaded_bytes,
                            total_bytes
                        }
                    )
                    | DiffUpdate::SophonPatcherUpdate(
                        SophonPatcherUpdate::DownloadingProgressBytes {
                            downloaded_bytes,
                            total_bytes
                        }
                    ) => {
                        tracing::debug!("Download progress [{downloaded_bytes}/{total_bytes}]");

                        self.fraction = downloaded_bytes as f64 / total_bytes as f64;

                        self.downloaded = Some((
                            prettify_bytes(downloaded_bytes),
                            prettify_bytes(total_bytes)
                        ));
                    }

                    // rest of sophon progress updates
                    DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::DeletingProgress {
                        deleted_files,
                        total_unused
                    }) => tracing::debug!(
                        "Deleted {deleted_files} unused files out of {total_unused}"
                    ),
                    DiffUpdate::SophonInstallerUpdate(
                        SophonInstallerUpdate::DownloadingProgressFiles {
                            downloaded_files,
                            total_files
                        }
                    ) => tracing::info!("Downloaded {downloaded_files} files out of {total_files}"),
                    DiffUpdate::SophonPatcherUpdate(SophonPatcherUpdate::PatchingProgress {
                        patched_files,
                        total_files
                    }) => tracing::info!("Patched {patched_files} files out of {total_files}")
                }
            }

            ProgressBarMsg::SetVisible(visible) => self.visible = visible
        }
    }
}
