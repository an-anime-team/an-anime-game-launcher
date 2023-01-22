use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use gtk::glib::clone;

use super::progress_bar::AppMsg as ProgressBarMsg;

use anime_launcher_sdk::config;
use anime_launcher_sdk::anime_game_core::installer::installer::*;

use std::path::PathBuf;

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

    pub show_recommended_only: bool,
    pub state: VersionState,

    pub progress_bar: Controller<super::ProgressBar>
}

#[derive(Debug)]
pub enum AppMsg {
    ShowRecommendedOnly(bool),
    PerformAction,
    SetState(VersionState)
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentVersion {
    type Init = (super::ComponentsListVersion, PathBuf);
    type Input = AppMsg;
    type Output = ();

    view! {
        row = adw::ActionRow {
            set_title: &model.title,

            #[watch]
            set_visible: !model.show_recommended_only || model.recommended,

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

                connect_clicked => AppMsg::PerformAction
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = ComponentVersion {
            name: init.0.name,
            title: init.0.title,
            recommended: init.0.recommended,

            download_uri: init.0.uri,
            download_folder: init.1,

            show_recommended_only: true,
            state: VersionState::NotDownloaded,

            progress_bar: super::ProgressBar::builder()
                .launch((None, false))
                .detach()
        };

        // Set default component state
        model.state = if model.download_folder.join(&model.name).exists() {
            VersionState::Downloaded
        } else {
            VersionState::NotDownloaded
        };

        // Set progress bar width
        model.progress_bar.widgets().progress_bar.set_width_request(200);

        let widgets = view_output!();

        widgets.row.add_suffix(model.progress_bar.widget());

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        tracing::debug!("Called component version [{}] event: {:?} (state = {:?})", self.title, msg, self.state);

        match msg {
            AppMsg::ShowRecommendedOnly(state) => self.show_recommended_only = state,

            AppMsg::PerformAction => {
                match self.state {
                    VersionState::Downloaded => {
                        let path = self.download_folder.join(&self.name);

                        if path.exists() {
                            // To hide main button while it's deleting compontent's folder
                            self.state = VersionState::Downloading;

                            // todo
                            std::fs::remove_dir_all(path).expect("Failed to delete component");

                            self.state = VersionState::NotDownloaded;
                        }
                    }

                    VersionState::NotDownloaded => {
                        if let Ok(config) = config::get() {
                            // todo
                            let mut installer = Installer::new(&self.download_uri).expect("Failed to create installer instance for this version");

                            if let Some(temp) = config.launcher.temp {
                                installer.set_temp_folder(temp);
                            }

                            self.state = VersionState::Downloading;

                            let progress_bar_sender = self.progress_bar.sender().clone();

                            #[allow(unused_must_use)]
                            std::thread::spawn(clone!(@strong self.download_folder as download_folder => move || {
                                progress_bar_sender.send(ProgressBarMsg::Reset);
                                progress_bar_sender.send(ProgressBarMsg::SetVisible(true));

                                installer.install(download_folder, move |state| {
                                    match &state {
                                        Update::UnpackingFinished |
                                        Update::DownloadingError(_) |
                                        Update::UnpackingError(_) => {
                                            progress_bar_sender.send(ProgressBarMsg::SetVisible(false));

                                            sender.input(AppMsg::SetState(if let Update::UnpackingFinished = &state {
                                                VersionState::Downloaded
                                            } else {
                                                VersionState::NotDownloaded
                                            }));
                                        },

                                        _ => ()
                                    }

                                    progress_bar_sender.send(ProgressBarMsg::UpdateFromState(state));
                                });
                            }));
                        }
                    }

                    _ => ()
                }
            }

            AppMsg::SetState(state) => self.state = state
        }
    }
}
