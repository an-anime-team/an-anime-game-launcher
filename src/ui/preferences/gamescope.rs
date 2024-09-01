use relm4::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;

use anime_launcher_sdk::config::schema_blanks::prelude::*;

use enum_ordinalize::Ordinalize;

use crate::*;

pub struct GamescopeApp;

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GamescopeApp {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::PreferencesWindow {
            set_title: Some(&tr!("gamescope")),

            set_modal: true,
            set_hide_on_close: true,

            // FIXME: doesn't work for any reason
            set_search_enabled: false,

            add = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    adw::ComboRow {
                        set_title: &tr!("window-mode"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &tr!("default"),
                            &tr!("borderless"),
                            &tr!("headless"),
                            &tr!("fullscreen")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.window_mode.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.window_mode = GamescopeWindowMode::from_ordinal_unsafe(row.selected() as i8);

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr!("game-resolution"),

                    adw::EntryRow {
                        set_title: &tr!("width"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.game_window.width {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    let value = row.text()
                                        .parse::<u64>()
                                        .unwrap_or_default();

                                    config.game.enhancements.gamescope.game_window.width = if value > 0 {
                                        Some(value)
                                    } else {
                                        None
                                    };

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::EntryRow {
                        set_title: &tr!("height"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.game_window.height {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    let value = row.text()
                                        .parse::<u64>()
                                        .unwrap_or_default();

                                    config.game.enhancements.gamescope.game_window.height = if value > 0 {
                                        Some(value)
                                    } else {
                                        None
                                    };

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr!("gamescope-resolution"),

                    adw::EntryRow {
                        set_title: &tr!("width"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.gamescope_window.width {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    let value = row.text()
                                        .parse::<u64>()
                                        .unwrap_or_default();

                                    config.game.enhancements.gamescope.gamescope_window.width = if value > 0 {
                                        Some(value)
                                    } else {
                                        None
                                    };

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::EntryRow {
                        set_title: &tr!("height"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.gamescope_window.height {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    let value = row.text()
                                        .parse::<u64>()
                                        .unwrap_or_default();

                                    config.game.enhancements.gamescope.gamescope_window.height = if value > 0 {
                                        Some(value)
                                    } else {
                                        None
                                    };

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr!("framerate"),

                    adw::EntryRow {
                        set_title: &tr!("framerate-limit"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.framerate.focused {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    let value = row.text()
                                        .parse::<u64>()
                                        .unwrap_or_default();

                                    config.game.enhancements.gamescope.framerate.focused = if value > 0 {
                                        Some(value)
                                    } else {
                                        None
                                    };

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::EntryRow {
                        set_title: &tr!("unfocused-framerate-limit"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.framerate.unfocused {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    let value = row.text()
                                        .parse::<u64>()
                                        .unwrap_or_default();

                                    config.game.enhancements.gamescope.framerate.unfocused = if value > 0 {
                                        Some(value)
                                    } else {
                                        None
                                    };

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr!("upscaling"),
                    set_description: Some(&tr!("upscaling-description")),

                    adw::ComboRow {
                        set_title: &tr!("upscaler"),
                        set_subtitle: &tr!("upscaler-description"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &tr!("none"),
                            &tr!("auto"),
                            &tr!("integer"),
                            &tr!("fit"),
                            &tr!("fill"),
                            &tr!("stretch")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.upscaling.upscaler.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.upscaling.upscaler = GamescopeUpscaler::from_ordinal_unsafe(row.selected() as i8);

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::ComboRow {
                        set_title: &tr!("upscale-filter"),
                        set_subtitle: &tr!("upscale-filter-description"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &tr!("none"),
                            &tr!("linear"),
                            &tr!("nearest"),
                            &tr!("fsr"),
                            &tr!("nis"),
                            &tr!("pixel")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.upscaling.filter.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.upscaling.filter = GamescopeUpscaleFilter::from_ordinal_unsafe(row.selected() as i8);

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::ComboRow {
                        set_title: &tr!("upscale-sharpness"),
                        set_subtitle: &tr!("upscale-sharpness-description"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &tr!("none"),
                            &tr!("smallest"),
                            &tr!("small"),
                            &tr!("balanced"),
                            &tr!("high"),
                            &tr!("highest")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.upscaling.sharpness.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.upscaling.sharpness = GamescopeUpscaleSharpness::from_ordinal_unsafe(row.selected() as i8);

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr!("options"),

                    adw::SwitchRow {
                        set_title: &tr!("hdr-support"),
                        set_subtitle: &tr!("hdr-support-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.hdr_support,

                        connect_active_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.options.hdr_support = switch.is_active();

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &tr!("realtime-scheduler"),
                        set_subtitle: &tr!("realtime-scheduler-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.realtime_scheduler,

                        connect_active_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.options.realtime_scheduler = switch.is_active();

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &tr!("adaptive-sync"),
                        set_subtitle: &tr!("adaptive-sync-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.adaptive_sync,

                        connect_active_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.options.adaptive_sync = switch.is_active();

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &tr!("force-grab-cursor"),
                        set_subtitle: &tr!("force-grab-cursor-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.force_grab_cursor,

                        connect_active_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.options.force_grab_cursor = switch.is_active();

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &tr!("mangohud"),
                        set_subtitle: &tr!("mangohud-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.mangohud,

                        connect_active_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.options.mangohud = switch.is_active();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr!("extra-args"),
                    set_description: Some(&tr!("extra-args-description")),

                    adw::EntryRow {
                        set_title: &tr!("extra-args"),

                        set_text: &CONFIG.game.enhancements.gamescope.extra_args,

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.extra_args = row.text().parse().unwrap_or_default();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing gamescope settings");

        let model = Self;
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
