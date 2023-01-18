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
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.wine.borderless,

                        connect_state_notify => move |switch| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.wine.borderless = switch.state();

                                    config::update(config);
                                }
                            }
                        }
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

                    set_selected: CONFIG.game.enhancements.hud.into(),

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.enhancements.hud = HUD::try_from(row.selected()).unwrap();

                                config::update(config);
                            }
                        }
                    }
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

                    // FSR strength selection
                    // 
                    // Ultra Quality = 5
                    // Quality       = 4
                    // Balanced      = 3
                    // Performance   = 2
                    // 
                    // Source: Bottles (https://github.com/bottlesdevs/Bottles/blob/22fa3573a13f4e9b9c429e4cdfe4ca29787a2832/src/ui/details-preferences.ui#L88)
                    set_selected: 5 - CONFIG.game.enhancements.fsr.strength as u32,

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.enhancements.fsr.strength = 5 - row.selected() as u64;

                                config::update(config);
                            }
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.fsr.enabled,

                        connect_state_notify => move |switch| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.fsr.enabled = switch.state();

                                    config::update(config);
                                }
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr("gamemode"),
                    set_subtitle: &tr("gamemode-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.gamemode,

                        connect_state_notify => move |switch| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamemode = switch.state();

                                    config::update(config);
                                }
                            }
                        }
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

                    set_selected: match Fps::from_num(CONFIG.game.enhancements.fps_unlocker.config.fps) {
                        Fps::Custom(_)         => 0,
                        Fps::Ninety            => 1,
                        Fps::HundredTwenty     => 2,
                        Fps::HundredFourtyFour => 3,
                        Fps::HundredSixtyFive  => 4,
                        Fps::HundredEighty     => 5,
                        Fps::TwoHundred        => 6,
                        Fps::TwoHundredFourty  => 7
                    },

                    connect_selected_notify => move |row| {
                        if is_ready() && row.selected() > 0 {
                            if let Ok(mut config) = config::get() {
                                config.game.enhancements.fps_unlocker.config.fps = Fps::list()[row.selected() as usize - 1].to_num();

                                config::update(config);
                            }
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.fps_unlocker.enabled,

                        connect_state_notify => move |switch| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.fps_unlocker.enabled = switch.state();

                                    config::update(config);
                                }
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr("power-saving"),
                    set_subtitle: &tr("power-saving-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.fps_unlocker.config.power_saving,

                        connect_state_notify => move |switch| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.fps_unlocker.config.power_saving = switch.state();

                                    config::update(config);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
