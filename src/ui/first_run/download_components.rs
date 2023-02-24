use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::components::wine;
use anime_launcher_sdk::components::dxvk;

use super::main::FirstRunAppMsg;
use crate::ui::components::*;

pub struct DownloadComponentsApp {
    progress_bar: AsyncController<ProgressBar>,

    wine_versions: Vec<wine::Version>,
    dxvk_versions: Vec<dxvk::Version>,

    downloading: bool
}

#[derive(Debug, Clone)]
pub enum DownloadComponentsAppMsg {
    Download,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DownloadComponentsApp {
    type Init = ();
    type Input = DownloadComponentsAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: "Download components",
                    add_css_class: "title-1"
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: !model.downloading,

                adw::ComboRow {
                    set_title: "Wine version",

                    #[watch]
                    set_model: Some(&gtk::StringList::new(model.wine_versions.iter()
                        .map(|version| version.title.as_ref())
                        .collect::<Vec<&str>>()
                        .as_slice()))
                },

                adw::ComboRow {
                    set_title: "DXVK version",

                    #[watch]
                    set_model: Some(&gtk::StringList::new(model.dxvk_versions.iter()
                        .map(|version| version.version.as_ref())
                        .collect::<Vec<&str>>()
                        .as_slice()))
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: !model.downloading,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 8,

                    gtk::Button {
                        set_label: "Download",
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => DownloadComponentsAppMsg::Download
                    },

                    gtk::Button {
                        set_label: "Exit",
                        add_css_class: "pill",

                        connect_clicked => DownloadComponentsAppMsg::Exit
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                #[watch]
                set_visible: model.downloading,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 20,
                    set_margin_top: 64,

                    append = model.progress_bar.widget(),
                }
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            progress_bar: ProgressBar::builder()
                .launch(ProgressBarInit {
                    caption: None,
                    display_progress: true,
                    display_fraction: true,
                    visible: true
                })
                .detach(),

            wine_versions: wine::get_groups()[0].versions.clone().into_iter().filter(|version| version.recommended).collect(),
            dxvk_versions: dxvk::get_groups()[0].versions.clone().into_iter().filter(|version| version.recommended).collect(),

            downloading: false
        };

        model.progress_bar.widget().set_width_request(360);

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            #[allow(unused_must_use)]
            DownloadComponentsAppMsg::Download => {
                self.downloading = true;

                // sender.output(Self::Output::ScrollToChooseVoiceovers);
            }

            DownloadComponentsAppMsg::Exit => {
                // TODO: relm4 has some function for it
                std::process::exit(0);
            }
        }
    }
}
