use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::config::prelude::*;

use crate::i18n::tr;
use crate::*;

pub struct GamescopeApp;

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GamescopeApp {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::PreferencesWindow {
            set_title: Some(&tr("gamescope")),

            set_modal: true,
            set_hide_on_close: true,

            // FIXME: doesn't work for any reason
            set_search_enabled: false,

            add = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    set_title: &tr("game-resolution"),

                    adw::EntryRow {
                        set_title: &tr("width"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &if CONFIG.game.enhancements.gamescope.game.width > 0 {
                            CONFIG.game.enhancements.gamescope.game.width.to_string()
                        } else {
                            String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamescope.game.width = row.text().parse().unwrap_or_default();
    
                                    config::update(config);
                                }
                            }
                        }
                    },

                    adw::EntryRow {
                        set_title: &tr("height"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &if CONFIG.game.enhancements.gamescope.game.height > 0 {
                            CONFIG.game.enhancements.gamescope.game.height.to_string()
                        } else {
                            String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamescope.game.height = row.text().parse().unwrap_or_default();
    
                                    config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr("gamescope-resolution"),

                    adw::EntryRow {
                        set_title: &tr("width"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &if CONFIG.game.enhancements.gamescope.gamescope.width > 0 {
                            CONFIG.game.enhancements.gamescope.gamescope.width.to_string()
                        } else {
                            String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamescope.gamescope.width = row.text().parse().unwrap_or_default();
    
                                    config::update(config);
                                }
                            }
                        }
                    },

                    adw::EntryRow {
                        set_title: &tr("height"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &if CONFIG.game.enhancements.gamescope.gamescope.height > 0 {
                            CONFIG.game.enhancements.gamescope.gamescope.height.to_string()
                        } else {
                            String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamescope.gamescope.height = row.text().parse().unwrap_or_default();
    
                                    config::update(config);
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr("upscaling"),

                    adw::ActionRow {
                        set_title: &tr("integer-scaling"),
                        set_subtitle: &tr("integer-scaling-description"),

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            set_state: CONFIG.game.enhancements.gamescope.integer_scaling,

                            connect_state_notify => |switch| {
                                if is_ready() {
                                    if let Ok(mut config) = config::get() {
                                        config.game.enhancements.gamescope.integer_scaling = switch.state();
        
                                        config::update(config);
                                    }
                                }
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "FSR",
                        set_subtitle: &tr("gamescope-fsr-description"),

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            set_state: CONFIG.game.enhancements.gamescope.fsr,

                            connect_state_notify => |switch| {
                                if is_ready() {
                                    if let Ok(mut config) = config::get() {
                                        config.game.enhancements.gamescope.fsr = switch.state();
        
                                        config::update(config);
                                    }
                                }
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Nvidia Image Scaling",
                        set_subtitle: &tr("nis-description"),

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            set_state: CONFIG.game.enhancements.gamescope.nis,

                            connect_state_notify => |switch| {
                                if is_ready() {
                                    if let Ok(mut config) = config::get() {
                                        config.game.enhancements.gamescope.nis = switch.state();
        
                                        config::update(config);
                                    }
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr("other-settings"),

                    // TODO: maybe use Fps enum like in fps unlocker settings

                    adw::EntryRow {
                        set_title: &tr("framerate-limit"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &if CONFIG.game.enhancements.gamescope.framerate.focused > 0 {
                            CONFIG.game.enhancements.gamescope.framerate.focused.to_string()
                        } else {
                            String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamescope.framerate.focused = row.text().parse().unwrap_or_default();
    
                                    config::update(config);
                                }
                            }
                        }
                    },

                    adw::EntryRow {
                        set_title: &tr("unfocused-framerate-limit"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &if CONFIG.game.enhancements.gamescope.framerate.unfocused > 0 {
                            CONFIG.game.enhancements.gamescope.framerate.unfocused.to_string()
                        } else {
                            String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamescope.framerate.unfocused = row.text().parse().unwrap_or_default();
    
                                    config::update(config);
                                }
                            }
                        }
                    },

                    adw::ComboRow {
                        set_title: &tr("window-mode"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &tr("borderless"),
                            &tr("fullscreen")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.window_type.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() {
                                if let Ok(mut config) = config::get() {
                                    config.game.enhancements.gamescope.window_type = WindowType::from_ordinal_unsafe(row.selected() as i8);
    
                                    config::update(config);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing gamescope settings");

        let model = Self;
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
