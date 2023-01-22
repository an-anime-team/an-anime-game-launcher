use relm4::prelude::*;

use adw::prelude::*;

use anime_launcher_sdk::anime_game_core::installer::installer::Update as InstallerUpdate;

use crate::prettify_bytes;

pub struct ProgressBar {
    pub fraction: f64,
    pub caption: Option<String>,

    /// e.g. (53.21 MB, 10 GB)
    pub downloaded: Option<(String, String)>,

    pub visible: bool
}

#[derive(Debug)]
pub enum AppMsg {
    Reset,
    UpdateCaption(Option<String>),

    /// (current bytes, total bytes) 
    UpdateProgress(u64, u64),

    UpdateFromState(InstallerUpdate),
    SetVisible(bool)
}

#[relm4::component(pub)]
impl SimpleComponent for ProgressBar {
    type Init = (Option<String>, bool);
    type Input = AppMsg;
    type Output = ();

    view! {
        progress_bar = gtk::ProgressBar {
            set_valign: gtk::Align::Center,

            #[watch]
            set_visible: model.visible,

            #[watch]
            set_fraction: model.fraction,

            #[watch]
            set_show_text: model.caption.is_some(),

            #[watch]
            set_text: match model.caption.as_ref() {
                Some(caption) => Some({
                    if let Some((curr, total)) = &model.downloaded {
                        // caption.push_str(&format!(": {:.2}% ({curr} of {total})", model.fraction));
                    }

                    caption
                }),
                None => None
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ProgressBar {
            fraction: 0.0,
            caption: init.0,
            downloaded: None,
            visible: init.1
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called components list event: {:?}", msg);

        match msg {
            AppMsg::Reset => {
                self.fraction = 0.0;
                self.downloaded = None;
                self.caption = None;
            }

            AppMsg::UpdateCaption(caption) => self.caption = caption,

            AppMsg::UpdateProgress(curr, total) => {
                self.fraction = curr as f64 / total as f64;

                self.downloaded = Some((
                    prettify_bytes(curr),
                    prettify_bytes(total)
                ));
            }

            // TODO: add translation
            AppMsg::UpdateFromState(state) => {
                match state {
                    InstallerUpdate::CheckingFreeSpace(_)  => self.caption = Some(String::from("Checking free space...")),
                    InstallerUpdate::DownloadingStarted(_) => self.caption = Some(String::from("Downloading...")),
                    InstallerUpdate::UnpackingStarted(_)   => self.caption = Some(String::from("Unpacking...")),

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

            AppMsg::SetVisible(visible) => self.visible = visible
        }
    }
}
