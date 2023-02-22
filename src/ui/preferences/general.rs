use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::config::launcher::LauncherStyle;
use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::components::*;
use anime_launcher_sdk::wincompatlib::prelude::*;

use crate::ui::components;
use crate::ui::components::*;
use crate::i18n::*;
use crate::*;

pub struct General {
    wine_components: AsyncController<ComponentsList<AppMsg>>,
    dxvk_components: AsyncController<ComponentsList<AppMsg>>,

    style: LauncherStyle,

    downloaded_wine_versions: Vec<wine::Version>,
    downloaded_dxvk_versions: Vec<dxvk::Version>,

    selected_wine_version: u32,
    selected_dxvk_version: u32,

    selecting_wine_version: bool,
    selecting_dxvk_version: bool
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Toast {
        title: String,
        description: Option<String>
    },
    UpdateLauncherStyle(LauncherStyle),
    WineRecommendedOnly(bool),
    DxvkRecommendedOnly(bool),
    UpdateDownloadedWine,
    UpdateDownloadedDxvk,
    SelectWine(usize),
    SelectDxvk(usize),
    ResetWineSelection(usize),
    ResetDxvkSelection(usize)
}

#[relm4::component(pub)]
impl SimpleComponent for General {
    type Init = ();
    type Input = AppMsg;
    type Output = super::main::AppMsg;

    view! {
        adw::PreferencesPage {
            set_title: &tr("general"),
            set_icon_name: Some("applications-system-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr("appearance"),

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,

                    set_spacing: 32,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        gtk::ToggleButton {
                            add_css_class: "card",

                            set_width_request: 180,
                            set_height_request: 120,

                            #[watch]
                            set_active: model.style == LauncherStyle::Modern,

                            gtk::Image {
                                set_from_resource: Some("/org/app/images/modern.svg")
                            },

                            connect_clicked => AppMsg::UpdateLauncherStyle(LauncherStyle::Modern)
                        },

                        gtk::Label {
                            set_text: &tr("modern"),

                            set_margin_top: 16
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        gtk::ToggleButton {
                            add_css_class: "card",

                            set_width_request: 180,
                            set_height_request: 120,

                            #[watch]
                            set_active: model.style == LauncherStyle::Classic,

                            gtk::Image {
                                set_from_resource: Some("/org/app/images/classic.svg")
                            },

                            connect_clicked => AppMsg::UpdateLauncherStyle(LauncherStyle::Classic)
                        },

                        gtk::Label {
                            set_text: &tr("classic"),

                            set_margin_top: 16
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                #[watch]
                set_visible: model.style == LauncherStyle::Classic,

                adw::ActionRow {
                    set_title: &tr("update-background"),
                    set_subtitle: &tr("update-background-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("general"),

                adw::ComboRow {
                    set_title: &tr("launcher-language"),
                    set_subtitle: &tr("launcher-language-description"),

                    // TODO: maybe simplify it by some way? e.g. specify such stuff in i18n mod

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        "English",
                        "Русский"
                    ]),

                    set_selected: match CONFIG.launcher.language.as_str() {
                        "en-us" => 0,
                        "ru-ru" => 1,
                        _ => 0
                    },

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.launcher.language = String::from(*[
                                    "en-us",
                                    "ru-ru"
                                ].get(row.selected() as usize).unwrap_or(&"en-us"));
    
                                config::update(config);
                            }
                        }
                    }
                },

                adw::ExpanderRow {
                    set_title: &tr("game-voiceovers"),

                    add_row = &adw::ActionRow {
                        set_title: &tr("english"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("japanese"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("korean"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("chinese"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_margin_top: 16,

                    gtk::Button {
                        set_label: &tr("repair-game")
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("status"),

                adw::ActionRow {
                    set_title: &tr("game-version"),

                    add_suffix = &gtk::Label {
                        set_text: &match GAME_DIFF.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(current) |
                                VersionDiff::Predownload { current, .. } |
                                VersionDiff::Diff { current, .. } |
                                VersionDiff::Outdated { current, .. } => current.to_string(),

                                VersionDiff::NotInstalled { .. } => tr("game-not-installed")
                            }

                            None => String::from("?")
                        },

                        add_css_class: match GAME_DIFF.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(_) => "success",
                                VersionDiff::Predownload { .. } => "accent",
                                VersionDiff::Diff { .. } => "warning",
                                VersionDiff::Outdated { .. } => "error",
                                VersionDiff::NotInstalled { .. } => ""
                            }

                            None => "success"
                        },

                        set_tooltip_text: Some(&match GAME_DIFF.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(_) => String::new(),
                                VersionDiff::Predownload { current, latest, .. } => tr_args("game-predownload-available", [
                                    ("old", current.to_string().into()),
                                    ("new", latest.to_string().into())
                                ]),
                                VersionDiff::Diff { current, latest, .. } => tr_args("game-update-available", [
                                    ("old", current.to_string().into()),
                                    ("new", latest.to_string().into())
                                ]),
                                VersionDiff::Outdated { latest, ..} => tr_args("game-outdated", [
                                    ("latest", latest.to_string().into())
                                ]),
                                VersionDiff::NotInstalled { .. } => String::new()
                            }

                            None => String::new()
                        })
                    }
                },

                adw::ActionRow {
                    set_title: &tr("patch-version"),

                    add_suffix = &gtk::Label {
                        set_text: &match PATCH.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => tr("patch-not-available"),
                                Patch::Outdated { current, .. } => tr_args("patch-outdated", [("current", current.to_string().into())]),
                                Patch::Preparation { .. } => tr("patch-preparation"),
                                Patch::Testing { version, .. } |
                                Patch::Available { version, .. } => version.to_string()
                            }

                            None => String::from("?")
                        },

                        add_css_class: match PATCH.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => "error",
                                Patch::Outdated { .. } |
                                Patch::Preparation { .. } |
                                Patch::Testing { .. } => "warning",
                                Patch::Available { .. } => unsafe {
                                    if let Ok(true) = PATCH.as_ref().unwrap_unchecked().is_applied(&CONFIG.game.path) {
                                        "success"
                                    } else {
                                        "warning"
                                    }
                                }
                            }

                            None => ""
                        },

                        set_tooltip_text: Some(&match PATCH.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => tr("patch-not-available-tooltip"),
                                Patch::Outdated { current, latest, .. } => tr_args("patch-outdated-tooltip", [
                                    ("current", current.to_string().into()),
                                    ("latest", latest.to_string().into())
                                ]),
                                Patch::Preparation { .. } => tr("patch-preparation-tooltip"),
                                Patch::Testing { .. } => tr("patch-testing-tooltip"),
                                Patch::Available { .. } => unsafe {
                                    if let Ok(true) = PATCH.as_ref().unwrap_unchecked().is_applied(&CONFIG.game.path) {
                                        String::new()
                                    } else {
                                        tr("patch-testing-tooltip")
                                    }
                                }
                            }

                            None => String::new()
                        })
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("wine-version"),

                adw::ComboRow {
                    set_title: &tr("selected-version"),

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

                adw::ActionRow {
                    set_title: &tr("recommended-only"),
                    set_subtitle: &tr("wine-recommended-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_state: true,

                        connect_state_notify[sender] => move |switch| {
                            sender.input(AppMsg::WineRecommendedOnly(switch.state()));
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                add = model.wine_components.widget(),
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("dxvk-version"),

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

                adw::ActionRow {
                    set_title: &tr("recommended-only"),
                    set_subtitle: &tr("dxvk-recommended-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_state: true,

                        connect_state_notify[sender] => move |switch| {
                            sender.input(AppMsg::DxvkRecommendedOnly(switch.state()));
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                add = model.dxvk_components.widget(),
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing about dialog");

        let model = Self {
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

            style: CONFIG.launcher.style,

            downloaded_wine_versions: vec![],
            downloaded_dxvk_versions: vec![],

            selected_wine_version: 0,
            selected_dxvk_version: 0,

            selecting_wine_version: false,
            selecting_dxvk_version: false
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        tracing::debug!("Called enhancements settings event: {:?}", msg);

        match msg {
            #[allow(unused_must_use)]
            AppMsg::UpdateLauncherStyle(style) => {
                if let Ok(mut config) = config::get() {
                    config.launcher.style = style;

                    config::update(config);
                }

                self.style = style;

                sender.output(Self::Output::UpdateLauncherStyle(style));
            }

            #[allow(unused_must_use)]
            AppMsg::Toast { title, description } => {
                sender.output(Self::Output::Toast { title, description });
            }

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
                                match wine.update_prefix(&config.game.wine.prefix) {
                                    Ok(_) => {
                                        config.game.wine.selected = Some(wine_name); 

                                        config::update(config);
                                    }

                                    Err(err) => {
                                        sender.input(AppMsg::Toast {
                                            title: tr("wine-prefix-update-failed"),
                                            description: Some(err.to_string())
                                        });
                                    }
                                }

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
                                    if let Err(err) = Dxvk::install(&wine, dxvk_folder, InstallParams::default()) {
                                        sender.input(AppMsg::Toast {
                                            title: tr("dxvk-install-failed"),
                                            description: Some(err.to_string())
                                        });
                                    }

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
