use relm4::prelude::*;

use adw::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::config::prelude::*;

use crate::i18n::tr;
use crate::*;

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

                    set_selected: CONFIG.game.wine.sync.ordinal() as u32,

                    connect_selected_notify => move |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.wine.sync = WineSync::from_ordinal_unsafe(row.selected() as i8);

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

                    set_selected: CONFIG.game.wine.language.ordinal() as u32,

                    connect_selected_notify => move |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.wine.language = WineLang::from_ordinal_unsafe(row.selected() as i8);

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
                        "960x540",
                        "1280x720",
                        "1920x1080",
                        "2560x1440",
                        "3840x2160",
                        &tr("custom")
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

                    set_selected: CONFIG.game.enhancements.hud.ordinal() as u32,

                    connect_selected_notify => move |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.enhancements.hud = HUD::from_ordinal_unsafe(row.selected() as i8);

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
                        "90",
                        "120",
                        "144",
                        "165",
                        "180",
                        "200",
                        "240",
                        &tr("custom")
                    ]),

                    set_selected: match Fps::from_num(CONFIG.game.enhancements.fps_unlocker.config.fps) {
                        Fps::Ninety            => 0,
                        Fps::HundredTwenty     => 1,
                        Fps::HundredFourtyFour => 2,
                        Fps::HundredSixtyFive  => 3,
                        Fps::HundredEighty     => 4,
                        Fps::TwoHundred        => 5,
                        Fps::TwoHundredFourty  => 6,

                        Fps::Custom(_) => 7
                    },

                    connect_selected_notify => move |row| {
                        if is_ready() && row.selected() < Fps::list().len() as u32 - 1 {
                            if let Ok(mut config) = config::get() {
                                config.game.enhancements.fps_unlocker.config.fps = Fps::list()[row.selected() as usize].to_num();

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
                },

                adw::ActionRow {
                    set_title: &tr("monitor"),
                    set_subtitle: &tr("monitor-description"),

                    add_suffix = &gtk::SpinButton {
                        set_valign: gtk::Align::Center,
                        set_adjustment: &gtk::Adjustment::new(1.0, 1.0, 10.0, 1.0, 1.0, 0.0),

                        set_value: CONFIG.game.enhancements.fps_unlocker.config.monitor as f64,

                        connect_changed => move |row| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.fps_unlocker.config.monitor = row.value() as u64;

                                    config::update(config);
                                }
                            }
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &tr("window-mode"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("default"),
                        &tr("popup"),
                        &tr("fullscreen")
                    ]),

                    set_selected: CONFIG.game.enhancements.fps_unlocker.config.window_mode.ordinal() as u32,

                    connect_selected_notify => move |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.enhancements.fps_unlocker.config.window_mode = WindowMode::from_ordinal_unsafe(row.selected() as i8);

                                config::update(config);
                            }
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &tr("priority"),
                    set_subtitle: &tr("priority-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("realtime"),
                        &tr("high"),
                        &tr("above-normal"),
                        &tr("normal"),
                        &tr("below-normal"),
                        &tr("low")
                    ]),

                    set_selected: CONFIG.game.enhancements.fps_unlocker.config.priority as u32,

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.game.enhancements.fps_unlocker.config.priority = row.selected() as u64;

                                config::update(config);
                            }
                        }
                    }
                },
            }
        }
    }
}
