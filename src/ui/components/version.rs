use std::path::PathBuf;

use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use gtk::glib::clone;

use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;

use super::ComponentGroupMsg;
use super::progress_bar::ProgressBarMsg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionState {
    Downloaded,
    Downloading,
    NotDownloaded
}

pub struct ComponentVersion {
    pub name: String,
    pub title: String,
    pub recommended: bool,

    pub download_uri: String,
    pub download_folder: PathBuf,
    pub download_filename: Option<String>,

    pub show_recommended_only: bool,
    pub state: VersionState,

    pub progress_bar: AsyncController<super::ProgressBar>
}

#[derive(Debug)]
pub enum ComponentVersionMsg {
    ShowRecommendedOnly(bool),
    PerformAction,
    SetState(VersionState)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for ComponentVersion {
    type Init = (super::ComponentsListVersion, PathBuf);
    type Input = ComponentVersionMsg;
    type Output = ComponentGroupMsg;

    view! {
        row = adw::ActionRow {
            set_title: &model.title,

            #[watch]
            set_visible: !model.show_recommended_only || model.recommended || model.state != VersionState::NotDownloaded,

            add_suffix = &gtk::Button {
                #[watch]
                set_icon_name: match model.state {
                    VersionState::NotDownloaded => "document-save-symbolic",

                    // In other states it will be downloaded or hidden
                    _ => "user-trash-symbolic"
                },

                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                #[watch]
                set_visible: model.state != VersionState::Downloading,

                connect_clicked => ComponentVersionMsg::PerformAction
            }
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut model = ComponentVersion {
            name: init.0.name.clone(),
            title: init.0.title,
            recommended: init.0.recommended,

            download_uri: init.0.uri,
            download_folder: init.1,
            download_filename: init.0.format.map(|format| format!("{}.{format}", init.0.name)),

            show_recommended_only: true,
            state: VersionState::NotDownloaded,

            progress_bar: super::ProgressBar::builder()
                .launch(super::ProgressBarInit {
                    caption: None,
                    display_progress: true,
                    display_fraction: false,
                    visible: false,
                })
                .detach()
        };

        // Set default component state
        model.state = if model.download_folder.join(&model.name).exists() {
            VersionState::Downloaded
        } else {
            VersionState::NotDownloaded
        };

        // Set progress bar width
        model.progress_bar.widget().set_width_request(200);

        let widgets = view_output!();

        widgets.row.add_suffix(model.progress_bar.widget());

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            ComponentVersionMsg::ShowRecommendedOnly(state) => self.show_recommended_only = state,

            ComponentVersionMsg::PerformAction => {
                match self.state {
                    VersionState::Downloaded => {
                        let path = self.download_folder.join(&self.name);

                        if path.exists() {
                            // To hide main button while it's deleting compontent's folder
                            self.state = VersionState::Downloading;

                            // todo
                            std::fs::remove_dir_all(path).expect("Failed to delete component");
                        }

                        self.state = VersionState::NotDownloaded;

                        #[allow(unused_must_use)] {
                            sender.output(ComponentGroupMsg::CallOnDeleted);
                        }
                    }

                    VersionState::NotDownloaded => {
                        if let Ok(config) = Config::get() {
                            // todo
                            let mut installer = Installer::new(&self.download_uri)
                                .expect("Failed to create installer instance for this version")
                                .with_temp_folder(config.launcher.temp.unwrap_or_else(std::env::temp_dir));

                            if let Some(filename) = &self.download_filename {
                                installer = installer.with_filename(filename.to_owned());
                            }

                            self.state = VersionState::Downloading;

                            let progress_bar_sender = self.progress_bar.sender().clone();

                            #[allow(unused_must_use)]
                            std::thread::spawn(clone!(@strong self.download_folder as download_folder => move || {
                                progress_bar_sender.send(ProgressBarMsg::Reset);
                                progress_bar_sender.send(ProgressBarMsg::SetVisible(true));

                                installer.install(download_folder, move |state| {
                                    match &state {
                                        InstallerUpdate::UnpackingFinished |
                                        InstallerUpdate::DownloadingError(_) |
                                        InstallerUpdate::UnpackingError(_) => {
                                            progress_bar_sender.send(ProgressBarMsg::SetVisible(false));

                                            if let InstallerUpdate::UnpackingFinished = &state {
                                                sender.input(ComponentVersionMsg::SetState(VersionState::Downloaded));
                                                sender.output(ComponentGroupMsg::CallOnDownloaded);
                                            }

                                            else {
                                                sender.input(ComponentVersionMsg::SetState(VersionState::NotDownloaded));
                                            }
                                        },

                                        _ => ()
                                    }

                                    progress_bar_sender.send(ProgressBarMsg::UpdateFromState(DiffUpdate::InstallerUpdate(state)));
                                });
                            }));
                        }
                    }

                    _ => ()
                }
            }

            ComponentVersionMsg::SetState(state) => self.state = state
        }
    }
}
