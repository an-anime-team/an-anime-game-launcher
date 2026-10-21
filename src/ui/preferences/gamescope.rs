// SPDX-License-Identifier: GPL-3.0-or-later
//
// an-anime-game-launcher
// Copyright (C) 2021 - 2026  Nikita Podvirnyi <krypt0nn@dawn.wine>
// Copyright (C) 2025 - 2026  @JohnTheCoolingFan <ivan8215145640@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
            set_title: Some(&i18n!("gamescope")),

            set_modal: true,
            set_hide_on_close: true,

            // FIXME: doesn't work for any reason
            set_search_enabled: false,

            add = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    adw::ComboRow {
                        set_title: &i18n!("window-mode"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &i18n!("default"),
                            &i18n!("borderless"),
                            &i18n!("headless"),
                            &i18n!("fullscreen")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.window_mode.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.window_mode = GamescopeWindowMode::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &i18n!("game-resolution"),

                    adw::EntryRow {
                        set_title: &i18n!("width"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.game_window.width {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
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
                    },

                    adw::EntryRow {
                        set_title: &i18n!("height"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.game_window.height {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
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
                },

                add = &adw::PreferencesGroup {
                    set_title: &i18n!("gamescope-resolution"),

                    adw::EntryRow {
                        set_title: &i18n!("width"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.gamescope_window.width {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
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
                    },

                    adw::EntryRow {
                        set_title: &i18n!("height"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.gamescope_window.height {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
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
                },

                add = &adw::PreferencesGroup {
                    set_title: &i18n!("framerate"),

                    adw::EntryRow {
                        set_title: &i18n!("framerate-limit"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.framerate.focused {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
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
                    },

                    adw::EntryRow {
                        set_title: &i18n!("unfocused-framerate-limit"),
                        set_input_purpose: gtk::InputPurpose::Digits,

                        set_text: &match CONFIG.game.enhancements.gamescope.framerate.unfocused {
                            Some(value) if value > 0 => value.to_string(),
                            _ => String::new()
                        },

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
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
                },

                add = &adw::PreferencesGroup {
                    set_title: &i18n!("upscaling"),
                    set_description: Some(&i18n!("upscaling-description")),

                    adw::ComboRow {
                        set_title: &i18n!("upscaler"),
                        set_subtitle: &i18n!("upscaler-description"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &i18n!("none"),
                            &i18n!("auto"),
                            &i18n!("integer"),
                            &i18n!("fit"),
                            &i18n!("fill"),
                            &i18n!("stretch")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.upscaling.upscaler.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.upscaling.upscaler = GamescopeUpscaler::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
                            }
                        }
                    },

                    adw::ComboRow {
                        set_title: &i18n!("upscale-filter"),
                        set_subtitle: &i18n!("upscale-filter-description"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &i18n!("none"),
                            &i18n!("linear"),
                            &i18n!("nearest"),
                            &i18n!("fsr"),
                            &i18n!("nis"),
                            &i18n!("pixel")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.upscaling.filter.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.upscaling.filter = GamescopeUpscaleFilter::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
                            }
                        }
                    },

                    adw::ComboRow {
                        set_title: &i18n!("upscale-sharpness"),
                        set_subtitle: &i18n!("upscale-sharpness-description"),

                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            &i18n!("none"),
                            &i18n!("smallest"),
                            &i18n!("small"),
                            &i18n!("balanced"),
                            &i18n!("high"),
                            &i18n!("highest")
                        ]),

                        set_selected: CONFIG.game.enhancements.gamescope.upscaling.sharpness.ordinal() as u32,

                        connect_selected_notify => |row| unsafe {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.upscaling.sharpness = GamescopeUpscaleSharpness::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &i18n!("options"),

                    adw::SwitchRow {
                        set_title: &i18n!("hdr-support"),
                        set_subtitle: &i18n!("hdr-support-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.hdr_support,

                        connect_active_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.options.hdr_support = switch.is_active();

                                Config::update(config);
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &i18n!("realtime-scheduler"),
                        set_subtitle: &i18n!("realtime-scheduler-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.realtime_scheduler,

                        connect_active_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.options.realtime_scheduler = switch.is_active();

                                Config::update(config);
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &i18n!("adaptive-sync"),
                        set_subtitle: &i18n!("adaptive-sync-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.adaptive_sync,

                        connect_active_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.options.adaptive_sync = switch.is_active();

                                Config::update(config);
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &i18n!("force-grab-cursor"),
                        set_subtitle: &i18n!("force-grab-cursor-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.force_grab_cursor,

                        connect_active_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.options.force_grab_cursor = switch.is_active();

                                Config::update(config);
                            }
                        }
                    },

                    adw::SwitchRow {
                        set_title: &i18n!("mangohud"),
                        set_subtitle: &i18n!("mangohud-description"),

                        set_active: CONFIG.game.enhancements.gamescope.options.mangohud,

                        connect_active_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.options.mangohud = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &i18n!("extra-args"),
                    set_description: Some(&i18n!("extra-args-description")),

                    adw::EntryRow {
                        set_title: &i18n!("extra-args"),

                        set_text: &CONFIG.game.enhancements.gamescope.extra_args,

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.extra_args = row.text().parse().unwrap_or_default();

                                Config::update(config);
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
