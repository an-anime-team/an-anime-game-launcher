use relm4::prelude::*;

use adw::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::config::prelude::*;

use crate::i18n::tr;
use crate::ui::main::is_ready;

lazy_static::lazy_static! {
    static ref CONFIG: config::Config = config::get().expect("Failed to load config");
}

#[relm4::widget_template(pub)]
impl WidgetTemplate for Enhancements {
    view! {
        adw::PreferencesPage {
            set_title: &tr("enhancements"),
            set_icon_name: Some("applications-graphics-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr("wine"),

                adw::ComboRow {
                    set_title: &tr("synchronization"),
                    set_subtitle: &tr("wine-sync-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("none"),
                        "ESync",
                        "FSync",
                        "Futex2"
                    ]),

                    set_selected: CONFIG.game.wine.sync.into(),

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.wine.sync = WineSync::try_from(row.selected()).unwrap();
    
                                config::update(config);
                            }
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &tr("language"),
                    set_subtitle: &tr("wine-lang-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("system"),
                        "English",
                        "Русский",
                        "Deutsch",
                        "Português",
                        "Polska",
                        "Français",
                        "Español",
                        "中国",
                        "日本語",
                        "한국어"
                    ]),

                    set_selected: CONFIG.game.wine.language.into(),

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.wine.language = WineLang::try_from(row.selected()).unwrap();
    
                                config::update(config);
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr("borderless-window"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                },

                adw::ComboRow {
                    set_title: &tr("virtual-desktop"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("custom"),
                        "960x540",
                        "1280x720",
                        "1920x1080",
                        "2560x1440",
                        "3840x2160"
                    ]),

                    set_selected: CONFIG.game.wine.virtual_desktop.get_resolution().into(),

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                let (width, height) = Resolution::try_from(row.selected()).unwrap().get_pair();

                                config.game.wine.virtual_desktop.width = width;
                                config.game.wine.virtual_desktop.height = height;
    
                                config::update(config);
                            }
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.wine.virtual_desktop.enabled,

                        connect_state_notify => move |switch| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.wine.virtual_desktop.enabled = switch.state();
        
                                    config::update(config);
                                }
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("game"),

                adw::ComboRow {
                    set_title: &tr("hud"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("none"),
                        "DXVK",
                        "MangoHud"
                    ]),
                },

                adw::ComboRow {
                    set_title: &tr("fsr"),
                    set_subtitle: &tr("fsr-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("ultra-quality"),
                        &tr("quality"),
                        &tr("balanced"),
                        &tr("performance")
                    ]),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                },

                adw::ActionRow {
                    set_title: &tr("gamemode"),
                    set_subtitle: &tr("gamemode-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("fps-unlocker"),

                adw::ComboRow {
                    set_title: &tr("enabled"),
                    set_subtitle: &tr("fps-unlocker-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("custom"),
                        "90",
                        "120",
                        "144",
                        "165",
                        "180",
                        "200",
                        "240"
                    ]),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                },

                adw::ActionRow {
                    set_title: &tr("power-saving"),
                    set_subtitle: &tr("power-saving-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                }
            }
        }
    }
}
