use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::anime_game_core::installer::installer::Update as InstallerUpdate;

use crate::prettify_bytes;
use crate::i18n::*;

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

    /// (current bytes, total bytes) 
    UpdateProgress(u64, u64),

    UpdateFromState(InstallerUpdate),
    SetVisible(bool)
}

#[relm4::component(async, pub)]
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

                    if model.display_fraction {
                        if let Some((curr, total)) = &model.downloaded {
                            caption = format!("{caption} ({curr} of {total})");
                        }
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
        _sender: AsyncComponentSender<Self>,
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

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            ProgressBarMsg::Reset => {
                self.fraction = 0.0;
                self.downloaded = None;
                self.caption = None;
            }

            ProgressBarMsg::UpdateCaption(caption) => self.caption = caption,

            ProgressBarMsg::UpdateProgress(curr, total) => {
                self.fraction = curr as f64 / total as f64;

                self.downloaded = Some((
                    prettify_bytes(curr),
                    prettify_bytes(total)
                ));
            }

            ProgressBarMsg::UpdateFromState(state) => {
                match state {
                    InstallerUpdate::CheckingFreeSpace(_)  => self.caption = Some(tr("checking-free-space")),
                    InstallerUpdate::DownloadingStarted(_) => self.caption = Some(tr("downloading")),
                    InstallerUpdate::UnpackingStarted(_)   => self.caption = Some(tr("unpacking")),

                    InstallerUpdate::DownloadingProgress(curr, total) |
                    InstallerUpdate::UnpackingProgress(curr, total) => {
                        self.fraction = curr as f64 / total as f64;

                        self.downloaded = Some((
                            prettify_bytes(curr),
                            prettify_bytes(total)
                        ));
                    }

                    InstallerUpdate::DownloadingFinished => tracing::info!("Downloading finished"),
                    InstallerUpdate::UnpackingFinished   => tracing::info!("Unpacking finished"),

                    InstallerUpdate::DownloadingError(err) => tracing::error!("Downloading error: {:?}", err),
                    InstallerUpdate::UnpackingError(err) => tracing::error!("Unpacking error: {:?}", err)
                }
            }

            ProgressBarMsg::SetVisible(visible) => self.visible = visible
        }
    }
}
