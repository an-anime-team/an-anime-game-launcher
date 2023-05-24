use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::wincompatlib::prelude::*;

use anime_launcher_sdk::components::*;
use anime_launcher_sdk::components::wine::WincompatlibWine;

use super::GeneralAppMsg;

use crate::ui::components;
use crate::ui::components::*;

use crate::i18n::*;
use crate::*;

pub struct ComponentsPage {
    wine_components: AsyncController<ComponentsList<ComponentsPageMsg>>,
    dxvk_components: AsyncController<ComponentsList<ComponentsPageMsg>>,

    downloaded_wine_versions: Vec<(wine::Version, wine::Features)>,
    downloaded_dxvk_versions: Vec<dxvk::Version>,
    allow_dxvk_selection: bool,

    selected_wine_version: u32,
    selected_dxvk_version: u32,

    selecting_wine_version: bool,
    selecting_dxvk_version: bool
}

#[derive(Debug, Clone)]
pub enum ComponentsPageMsg {
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
impl SimpleAsyncComponent for ComponentsPage {
    type Init = ();
    type Input = ComponentsPageMsg;
    type Output = GeneralAppMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: &tr("components")
                },

                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",

                    connect_clicked[sender] => move |_| {
                        sender.output(GeneralAppMsg::OpenMainPage).unwrap();
                    }
                }
            },

            adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    set_title: &tr("wine-version"),

                    adw::ComboRow {
                        set_title: &tr("selected-version"),

                        #[watch]
                        #[block_signal(wine_selected_notify)]
                        set_model: Some(&gtk::StringList::new(&model.downloaded_wine_versions.iter().map(|(version, _)| version.title.as_str()).collect::<Vec<&str>>())),

                        #[watch]
                        #[block_signal(wine_selected_notify)]
                        set_selected: model.selected_wine_version,

                        #[watch]
                        set_activatable: !model.selecting_wine_version,

                        connect_selected_notify[sender] => move |row| {
                            if is_ready() {
                                sender.input(ComponentsPageMsg::SelectWine(row.selected() as usize));
                            }
                        } @wine_selected_notify,

                        add_suffix = &gtk::Spinner {
                            set_spinning: true,

                            #[watch]
                            set_visible: model.selecting_wine_version
                        }
                    },

                    adw::ActionRow {
                        set_title: &tr("recommended-only"),
                        set_subtitle: &tr("wine-recommended-description"),

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,

                            #[block_signal(wine_recommended_notify)]
                            set_state: true,

                            connect_state_notify[sender] => move |switch| {
                                if is_ready() {
                                    sender.input(ComponentsPageMsg::WineRecommendedOnly(switch.state()));
                                }
                            } @wine_recommended_notify
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    add = model.wine_components.widget(),
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr("wine-options"),

                    adw::ActionRow {
                        set_title: &tr("wine-use-shared-libraries"),
                        set_subtitle: &tr("wine-use-shared-libraries-description"),

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,

                            #[block_signal(wine_shared_libraries_notify)]
                            set_state: CONFIG.game.wine.shared_libraries.wine,

                            connect_state_notify => |switch| {
                                if is_ready() {
                                    if let Ok(mut config) = Config::get() {
                                        config.game.wine.shared_libraries.wine = switch.state();

                                        Config::update(config);
                                    }
                                }
                            } @wine_shared_libraries_notify
                        }
                    },

                    adw::ActionRow {
                        set_title: &tr("gstreamer-use-shared-libraries"),
                        set_subtitle: &tr("gstreamer-use-shared-libraries-description"),

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,

                            #[block_signal(gstreamer_shared_libraries_notify)]
                            set_state: CONFIG.game.wine.shared_libraries.gstreamer,

                            connect_state_notify => |switch| {
                                if is_ready() {
                                    if let Ok(mut config) = Config::get() {
                                        config.game.wine.shared_libraries.gstreamer = switch.state();

                                        Config::update(config);
                                    }
                                }
                            } @gstreamer_shared_libraries_notify
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr("dxvk-version"),

                    #[watch]
                    set_description: Some(&if !model.allow_dxvk_selection {
                        tr("dxvk-selection-disabled")
                    } else {
                        String::new()
                    }),

                    #[watch]
                    set_sensitive: model.allow_dxvk_selection,

                    adw::ComboRow {
                        set_title: &tr("selected-version"),
    
                        #[watch]
                        #[block_signal(dxvk_selected_notify)]
                        set_model: Some(&gtk::StringList::new(&model.downloaded_dxvk_versions.iter().map(|version| version.name.as_str()).collect::<Vec<&str>>())),

                        #[watch]
                        #[block_signal(dxvk_selected_notify)]
                        set_selected: model.selected_dxvk_version,

                        #[watch]
                        set_activatable: !model.selecting_dxvk_version,

                        connect_selected_notify[sender] => move |row| {
                            if is_ready() {
                                sender.input(ComponentsPageMsg::SelectDxvk(row.selected() as usize));
                            }
                        } @dxvk_selected_notify,

                        add_suffix = &gtk::Spinner {
                            set_spinning: true,

                            #[watch]
                            set_visible: model.selecting_dxvk_version
                        }
                    },

                    adw::ActionRow {
                        set_title: &tr("recommended-only"),
                        set_subtitle: &tr("dxvk-recommended-description"),

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,

                            #[block_signal(dxvk_recommended_notify)]
                            set_state: true,

                            connect_state_notify[sender] => move |switch| {
                                if is_ready() {
                                    sender.input(ComponentsPageMsg::DxvkRecommendedOnly(switch.state()));
                                }
                            } @dxvk_recommended_notify
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    #[watch]
                    set_sensitive: model.allow_dxvk_selection,

                    add = model.dxvk_components.widget(),
                },
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing general settings -> components page");

        let model = Self {
            wine_components: ComponentsList::builder()
                .launch(ComponentsListInit {
                    pattern: ComponentsListPattern {
                        download_folder: CONFIG.game.wine.builds.clone(),
                        groups: wine::get_groups(&CONFIG.components.path).unwrap_or_default()
                            .into_iter()
                            .map(|mut group| {
                                group.versions = group.versions.into_iter().take(12).collect();

                                let mut group: ComponentsListGroup = group.into();
                                let mut recommended = 6;

                                for i in 0..group.versions.len() {
                                    if recommended > 0 && group.versions[i].recommended {
                                        recommended -= 1;
                                    }

                                    else {
                                        group.versions[i].recommended = false;
                                    }
                                }

                                group
                            })
                            .collect()
                    },
                    on_downloaded: Some(ComponentsPageMsg::UpdateDownloadedWine),
                    on_deleted: Some(ComponentsPageMsg::UpdateDownloadedWine)
                })
                .forward(sender.input_sender(), std::convert::identity),

            dxvk_components: ComponentsList::builder()
                .launch(ComponentsListInit {
                    pattern: ComponentsListPattern {
                        download_folder: CONFIG.game.dxvk.builds.clone(),
                        groups: dxvk::get_groups(&CONFIG.components.path).unwrap_or_default()
                            .into_iter()
                            .map(|mut group| {
                                group.versions = group.versions.into_iter().take(12).collect();

                                let mut group: ComponentsListGroup = group.into();
                                let mut recommended = 6;

                                for i in 0..group.versions.len() {
                                    if recommended > 0 && group.versions[i].recommended {
                                        recommended -= 1;
                                    }

                                    else {
                                        group.versions[i].recommended = false;
                                    }
                                }

                                group
                            })
                            .collect()
                    },
                    on_downloaded: Some(ComponentsPageMsg::UpdateDownloadedDxvk),
                    on_deleted: Some(ComponentsPageMsg::UpdateDownloadedDxvk)
                })
                .forward(sender.input_sender(), std::convert::identity),

            downloaded_wine_versions: vec![],
            downloaded_dxvk_versions: vec![],

            allow_dxvk_selection: match &CONFIG.game.wine.selected {
                Some(version) => match wine::Group::find_in(&CONFIG.components.path, version) {
                    Ok(Some(group)) => group.features.unwrap_or_default().need_dxvk,
                    _ => true
                }

                None => true
            },

            selected_wine_version: 0,
            selected_dxvk_version: 0,

            selecting_wine_version: false,
            selecting_dxvk_version: false
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        tracing::debug!("Called general settings event: {:?}", msg);

        match msg {
            ComponentsPageMsg::WineRecommendedOnly(state) => {
                // todo
                self.wine_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }

            ComponentsPageMsg::DxvkRecommendedOnly(state) => {
                // todo
                self.dxvk_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }

            ComponentsPageMsg::UpdateDownloadedWine => {
                self.downloaded_wine_versions = wine::get_downloaded(&CONFIG.components.path, &CONFIG.game.wine.builds)
                    .unwrap_or_default()
                    .into_iter()
                    .flat_map(|group| group.versions.clone().into_iter()
                        .map(move |version| {
                            let features = version.features_in(&group).unwrap_or_default();

                            (version, features)
                        })
                    ).collect();

                self.selected_wine_version = if let Some(selected) = &CONFIG.game.wine.selected {
                    let mut index = 0;

                    for (i, (version, _)) in self.downloaded_wine_versions.iter().enumerate() {
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

            ComponentsPageMsg::UpdateDownloadedDxvk => {
                self.downloaded_dxvk_versions = dxvk::get_downloaded(&CONFIG.components.path, &CONFIG.game.dxvk.builds)
                    .unwrap_or_default()
                    .into_iter()
                    .flat_map(|group| group.versions)
                    .collect();

                self.selected_dxvk_version = if let Ok(Some(selected)) = CONFIG.get_selected_dxvk() {
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

            ComponentsPageMsg::SelectWine(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some((version, features)) = self.downloaded_wine_versions.get(index) {
                        if config.game.wine.selected.as_ref() != Some(&version.title) {
                            self.selecting_wine_version = true;
                            self.allow_dxvk_selection = features.need_dxvk;

                            let wine = version
                                .to_wine(&config.components.path, Some(&config.game.wine.builds.join(&version.name)))
                                .with_prefix(&config.game.wine.prefix)
                                .with_loader(WineLoader::Current)
                                .with_arch(WineArch::Win64);

                            let wine_name = version.name.to_string();

                            std::thread::spawn(move || {
                                match wine.update_prefix::<&str>(None) {
                                    Ok(_) => {
                                        config.game.wine.selected = Some(wine_name); 

                                        Config::update(config);
                                    }

                                    Err(err) => {
                                        sender.output(GeneralAppMsg::Toast {
                                            title: tr("wine-prefix-update-failed"),
                                            description: Some(err.to_string())
                                        }).unwrap();
                                    }
                                }

                                sender.input(ComponentsPageMsg::ResetWineSelection(index));
                            });
                        }
                    }
                }
            }

            ComponentsPageMsg::ResetWineSelection(index) => {
                self.selecting_wine_version = false;
                self.selected_wine_version = index as u32;
            }

            ComponentsPageMsg::SelectDxvk(index) => {
                if let Ok(config) = Config::get() {
                    if let Some(version) = self.downloaded_dxvk_versions.get(index) {
                        if let Ok(selected) = config.get_selected_dxvk() {
                            if selected.is_none() || selected.unwrap().name != version.name {
                                self.selecting_dxvk_version = true;

                                let mut wine = match config.get_selected_wine() {
                                    Ok(Some(version)) => {
                                        match version.to_wine(config.components.path, Some(config.game.wine.builds.join(&version.name))) {
                                            WincompatlibWine::Default(wine) => wine,
                                            WincompatlibWine::Proton(_) => return
                                        }
                                    }

                                    _ => Wine::default()
                                };

                                wine = wine.with_prefix(config.game.wine.prefix);

                                let dxvk_folder = config.game.dxvk.builds.join(&version.name);

                                std::thread::spawn(move || {
                                    if let Err(err) = Dxvk::install(&wine, dxvk_folder, InstallParams::default()) {
                                        sender.output(GeneralAppMsg::Toast {
                                            title: tr("dxvk-install-failed"),
                                            description: Some(err.to_string())
                                        }).unwrap();
                                    }

                                    sender.input(ComponentsPageMsg::ResetDxvkSelection(index));
                                });
                            }
                        }
                    }
                }
            }

            ComponentsPageMsg::ResetDxvkSelection(index) => {
                self.selecting_dxvk_version = false;
                self.selected_dxvk_version = index as u32;
            }
        }
    }
}
