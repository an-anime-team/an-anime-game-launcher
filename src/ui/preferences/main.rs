use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::components::*;
use anime_launcher_sdk::wincompatlib::prelude::*;

use crate::ui::components::{self, *};
use crate::i18n::tr;
use crate::is_ready;

use crate::CONFIG;

pub struct App {
    wine_components: AsyncController<ComponentsList>,
    dxvk_components: AsyncController<ComponentsList>,

    downloaded_wine_versions: Vec<wine::Version>,
    downloaded_dxvk_versions: Vec<dxvk::Version>,

    selected_wine_version: u32,
    selected_dxvk_version: u32,

    selecting_wine_version: bool,
    selecting_dxvk_version: bool
}

#[derive(Debug, Clone, Copy)]
pub enum AppMsg {
    WineRecommendedOnly(bool),
    DxvkRecommendedOnly(bool),
    UpdateDownloadedWine,
    UpdateDownloadedDxvk,
    SelectWine(usize),
    SelectDxvk(usize),
    ResetWineSelection(usize),
    ResetDxvkSelection(usize)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for App {
    type Init = gtk::Window;
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        preferences_window = adw::PreferencesWindow {
            set_title: Some(&tr("preferences")),
            set_default_size: (700, 560),
            set_hide_on_close: true,
            set_modal: true,

            #[template]
            add = &super::general::General {
                // Here technically it's AdwPreferencesGroup inside of AdwPreferencesGroup
                // but I have no idea how to do it other way
                // There're no graphical glitches so don't care

                #[template_child]
                wine_versions {
                    add = model.wine_components.widget(),
                },

                #[template_child]
                wine_version_selector {
                    #[watch]
                    #[block_signal(wine_selected_notify)]
                    set_model: Some(&gtk::StringList::new(&model.downloaded_wine_versions.iter().map(|version| version.title.as_str()).collect::<Vec<&str>>())),

                    #[watch]
                    #[block_signal(wine_selected_notify)]
                    set_selected: model.selected_wine_version,

                    #[watch]
                    set_activatable: !model.selecting_wine_version,

                    #[watch]
                    set_icon_name: if model.selecting_wine_version {
                        Some("process-working")
                    } else {
                        None
                    },

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            sender.input(AppMsg::SelectWine(row.selected() as usize));
                        }
                    } @wine_selected_notify
                },

                #[template_child]
                wine_recommended_only {
                    connect_state_notify[sender] => move |switch| {
                        sender.input(AppMsg::WineRecommendedOnly(switch.state()));
                    }
                },

                #[template_child]
                dxvk_versions {
                    add = model.dxvk_components.widget(),
                },

                #[template_child]
                dxvk_version_selector {
                    #[watch]
                    #[block_signal(dxvk_selected_notify)]
                    set_model: Some(&gtk::StringList::new(&model.downloaded_dxvk_versions.iter().map(|version| version.name.as_str()).collect::<Vec<&str>>())),

                    #[watch]
                    #[block_signal(dxvk_selected_notify)]
                    set_selected: model.selected_dxvk_version,

                    #[watch]
                    set_activatable: !model.selecting_dxvk_version,

                    #[watch]
                    set_icon_name: if model.selecting_dxvk_version {
                        Some("process-working")
                    } else {
                        None
                    },

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            sender.input(AppMsg::SelectDxvk(row.selected() as usize));
                        }
                    } @dxvk_selected_notify
                },

                #[template_child]
                dxvk_recommended_only {
                    connect_state_notify[sender] => move |switch| {
                        sender.input(AppMsg::DxvkRecommendedOnly(switch.state()));
                    }
                },
            },

            #[template]
            add = &super::enhancements::Enhancements,
            
            connect_close_request => |_| {
                anime_launcher_sdk::config::flush().unwrap(); // FIXME

                gtk::Inhibit::default()
            }
        }
    }

    #[allow(clippy::redundant_clone)]
    async fn init(
        parent: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing preferences window");

        let model = App {
            wine_components: ComponentsList::builder()
                .launch(ComponentsListInit {
                    pattern: ComponentsListPattern {
                        download_folder: CONFIG.game.wine.builds.clone(),
                        groups: wine::get_groups().into_iter().map(|group| group.into()).collect()
                    },
                    on_downloaded: Some(AppMsg::UpdateDownloadedWine),
                    on_deleted: Some(AppMsg::UpdateDownloadedWine)
                })
                .forward(sender.input_sender(), std::convert::identity),

            dxvk_components: ComponentsList::builder()
                .launch(ComponentsListInit {
                    pattern: ComponentsListPattern {
                        download_folder: CONFIG.game.dxvk.builds.clone(),
                        groups: dxvk::get_groups().into_iter().map(|group| group.into()).collect()
                    },
                    on_downloaded: Some(AppMsg::UpdateDownloadedDxvk),
                    on_deleted: Some(AppMsg::UpdateDownloadedDxvk)
                })
                .forward(sender.input_sender(), std::convert::identity),

            downloaded_wine_versions: vec![],
            downloaded_dxvk_versions: vec![],

            selected_wine_version: 0,
            selected_dxvk_version: 0,

            selecting_wine_version: false,
            selecting_dxvk_version: false
        };

        let widgets = view_output!();

        widgets.preferences_window.set_transient_for(Some(&parent));

        sender.input(AppMsg::UpdateDownloadedWine);
        sender.input(AppMsg::UpdateDownloadedDxvk);

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        tracing::debug!("Called preferences window event: {:?}", msg);

        match msg {
            AppMsg::WineRecommendedOnly(state) => {
                // todo
                self.wine_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }

            AppMsg::DxvkRecommendedOnly(state) => {
                // todo
                self.dxvk_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }

            AppMsg::UpdateDownloadedWine => {
                self.downloaded_wine_versions = wine::get_downloaded(&CONFIG.game.wine.builds).unwrap_or_default();

                self.selected_wine_version = if let Some(selected) = &CONFIG.game.wine.selected {
                    let mut index = 0;
        
                    for (i, version) in self.downloaded_wine_versions.iter().enumerate() {
                        if &version.name == selected {
                            index = i;
        
                            break;
                        }
                    }
        
                    index as u32
                }

                else {
                    0
                };
            }

            AppMsg::UpdateDownloadedDxvk => {
                self.downloaded_dxvk_versions = dxvk::get_downloaded(&CONFIG.game.dxvk.builds).unwrap_or_default();

                self.selected_dxvk_version = if let Ok(Some(selected)) = CONFIG.try_get_selected_dxvk_info() {
                    let mut index = 0;
        
                    for (i, version) in self.downloaded_dxvk_versions.iter().enumerate() {
                        if version.name == selected.name {
                            index = i;
        
                            break;
                        }
                    }
        
                    index as u32
                }

                else {
                    0
                };
            }

            AppMsg::SelectWine(index) => {
                if let Ok(mut config) = config::get() {
                    if let Some(version) = self.downloaded_wine_versions.get(index) {
                        if config.game.wine.selected.as_ref().unwrap_or(&String::new()) != &version.title {
                            self.selecting_wine_version = true;

                            let wine = version.to_wine(Some(config.game.wine.builds.join(&version.name)));
                            let wine_name = version.name.to_string();

                            std::thread::spawn(move || {
                                wine.update_prefix(&config.game.wine.prefix)
                                    .expect("Failed to update wine prefix");

                                config.game.wine.selected = Some(wine_name); 

                                config::update(config);

                                sender.input(AppMsg::ResetWineSelection(index));
                            });
                        }
                    }
                }
            }

            AppMsg::ResetWineSelection(index) => {
                self.selecting_wine_version = false;
                self.selected_wine_version = index as u32;
            }

            AppMsg::SelectDxvk(index) => {
                if let Ok(config) = config::get() {
                    if let Some(version) = self.downloaded_dxvk_versions.get(index) {
                        if let Ok(selected) = config.try_get_selected_dxvk_info() {
                            if selected.is_none() || selected.unwrap().name != version.name {
                                self.selecting_dxvk_version = true;

                                let mut wine = match config.try_get_selected_wine_info() {
                                    Some(version) => version.to_wine(Some(config.game.wine.builds.join(&version.name))),
                                    None => Wine::default()
                                };

                                wine = wine.with_prefix(config.game.wine.prefix);

                                let dxvk_folder = config.game.dxvk.builds.join(&version.name);

                                std::thread::spawn(move || {
                                    Dxvk::install(&wine, dxvk_folder, InstallParams::default())
                                        .expect("Failed to install dxvk");

                                    sender.input(AppMsg::ResetDxvkSelection(index));
                                });
                            }
                        }
                    }
                }
            }

            AppMsg::ResetDxvkSelection(index) => {
                self.selecting_dxvk_version = false;
                self.selected_dxvk_version = index as u32;
            }
        }
    }
}
