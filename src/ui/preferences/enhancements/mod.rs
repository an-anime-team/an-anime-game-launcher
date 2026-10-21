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

use anime_launcher_sdk::is_available;

use enum_ordinalize::Ordinalize;

pub mod game;
pub mod sandbox;
pub mod environment;

use game::*;
use sandbox::*;
use environment::*;

use crate::*;

use super::gamescope::*;
use super::main::PreferencesAppMsg;

pub struct EnhancementsApp {
    gamescope: AsyncController<GamescopeApp>,
    game_page: AsyncController<GamePage>,
    sandbox_page: AsyncController<SandboxPage>,
    environment_page: AsyncController<EnvironmentPage>,
    timeout_fix: bool
}

#[derive(Debug)]
pub enum EnhancementsAppMsg {
    SetGamescopeParent,
    OpenGamescope,
    OpenMainPage,
    OpenGameSettingsPage,
    OpenSandboxSettingsPage,
    OpenEnvironmentSettingsPage,

    SetTimeoutFix(bool),

    Toast {
        title: String,
        description: Option<String>
    }
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for EnhancementsApp {
    type Init = ();
    type Input = EnhancementsAppMsg;
    type Output = PreferencesAppMsg;

    view! {
        #[root]
        adw::PreferencesPage {
            set_title: &i18n!("enhancements"),
            set_icon_name: Some("applications-graphics-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &i18n!("options"),

                adw::ActionRow {
                    set_title: &i18n!("game"),
                    set_subtitle: &i18n!("game-settings-description"),

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => EnhancementsAppMsg::OpenGameSettingsPage
                },

                adw::ActionRow {
                    set_title: &i18n!("sandbox"),
                    set_subtitle: &i18n!("sandbox-settings-description"),

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => EnhancementsAppMsg::OpenSandboxSettingsPage
                },

                adw::ActionRow {
                    set_title: &i18n!("environment"),
                    set_subtitle: &i18n!("environment-settings-description"),

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => EnhancementsAppMsg::OpenEnvironmentSettingsPage
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &i18n!("wine"),

                adw::ComboRow {
                    set_title: &i18n!("synchronization"),
                    set_subtitle: &i18n!("wine-sync-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &i18n!("none"),
                        "ESync",
                        "FSync",
                        "NTSync"
                    ]),

                    set_selected: CONFIG.game.wine.sync.ordinal() as u32,

                    connect_selected_notify => |row| unsafe {
                        if is_ready() && let Ok(mut config) = Config::get() {
                            config.game.wine.sync = WineSync::from_ordinal_unsafe(row.selected() as i8);

                            Config::update(config);
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &i18n!("language"),
                    set_subtitle: &i18n!("wine-lang-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &i18n!("system"),
                        "English",
                        "Русский",
                        "Deutsch",
                        "Português",
                        "Polska",
                        "Français",
                        "Español",
                        "中国",
                        "日本語",
                        "한국어",
                        "Indonesia"
                    ]),

                    set_selected: CONFIG.game.wine.language.ordinal() as u32,

                    connect_selected_notify => |row| unsafe {
                        if is_ready() && let Ok(mut config) = Config::get() {
                            config.game.wine.language = WineLang::from_ordinal_unsafe(row.selected() as i8);

                            Config::update(config);
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("borderless-window"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.wine.borderless,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.wine.borderless = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                #[name = "winewayland_row"]
                adw::ActionRow {
                    set_title: &i18n!("winewayland"),
                    set_subtitle: &i18n!("winewayland-description"),

                    set_sensitive: is_wayland_available(),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.wine.winewayland,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.wine.winewayland = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &i18n!("virtual-desktop"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        "960x540",
                        "1280x720",
                        "1920x1080",
                        "2560x1440",
                        "3840x2160",
                        &i18n!("custom")
                    ]),

                    set_selected: CONFIG.game.wine.virtual_desktop.get_resolution().into(),

                    connect_selected_notify => |row| {
                        if is_ready() && let Ok(mut config) = Config::get() {
                            let (width, height) = Resolution::try_from(row.selected()).unwrap().get_pair();

                            config.game.wine.virtual_desktop.width = width;
                            config.game.wine.virtual_desktop.height = height;

                            Config::update(config);
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.wine.virtual_desktop.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.wine.virtual_desktop.enabled = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("timeout-fix"),
                    set_subtitle: &i18n!("timeout-fix-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        #[watch]
                        set_active: model.timeout_fix,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.wine.timeout_fix = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("map-drive-c"),
                    set_subtitle: &i18n!("map-drive-c-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.wine.drives.drive_c,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.wine.drives.drive_c = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                #[name = "map_game_folder_row"]
                adw::ComboRow {
                    set_title: &i18n!("map-game-folder"),
                    set_subtitle: &i18n!("map-game-folder-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&AllowedDrives::list().iter()
                        .map(|drive| drive.to_drive())
                        .collect::<Vec<_>>()),

                    set_selected: match CONFIG.game.wine.drives.game_folder {
                        Some(drive) => AllowedDrives::list().iter()
                            .position(|allowed| *allowed == drive)
                            .unwrap_or(8) as u32,

                        None => 8 // G:
                    },

                    connect_selected_notify => |row| {
                        if is_ready() && let Ok(mut config) = Config::get() {
                            config.game.wine.drives.game_folder = Some(AllowedDrives::list()[row.selected() as usize]);

                            Config::update(config);
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.wine.drives.game_folder.is_some(),

                        connect_state_notify[map_game_folder_row] => move |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                if switch.is_active() {
                                    config.game.wine.drives.game_folder = Some(AllowedDrives::list()[map_game_folder_row.selected() as usize]);
                                } else {
                                    config.game.wine.drives.game_folder = None;
                                }

                                Config::update(config);
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &i18n!("game"),

                adw::ComboRow {
                    set_title: &i18n!("hud"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &i18n!("none"),
                        "DXVK",
                        "MangoHud"
                    ]),

                    set_selected: CONFIG.game.enhancements.hud.ordinal() as u32,

                    connect_selected_notify => |row| unsafe {
                        if is_ready() && let Ok(mut config) = Config::get() {
                            config.game.enhancements.hud = HUD::from_ordinal_unsafe(row.selected() as i8);

                            Config::update(config);
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &i18n!("fsr"),
                    set_subtitle: &i18n!("fsr-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &i18n!("ultra-quality"),
                        &i18n!("quality"),
                        &i18n!("balanced"),
                        &i18n!("performance")
                    ]),

                    set_selected: CONFIG.game.enhancements.fsr.quality.ordinal() as u32,

                    connect_selected_notify => |row| unsafe {
                        if is_ready() && let Ok(mut config) = Config::get() {
                            config.game.enhancements.fsr.quality = FsrQuality::from_ordinal_unsafe(row.selected() as i8);

                            Config::update(config);
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.enhancements.fsr.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.fsr.enabled = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("gamemode"),
                    set_subtitle: &i18n!("gamemode-description"),

                    set_sensitive: is_available("gamemoderun"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.enhancements.gamemode,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamemode = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("gamescope"),
                    set_subtitle: &i18n!("gamescope-description"),

                    set_sensitive: is_available("gamescope"),

                    add_suffix = &gtk::Button {
                        set_icon_name: "emblem-system-symbolic",
                        add_css_class: "flat",

                        set_valign: gtk::Align::Center,

                        connect_clicked => EnhancementsAppMsg::OpenGamescope
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.enhancements.gamescope.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.gamescope.enabled = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &i18n!("fps-unlocker"),

                adw::ComboRow {
                    set_title: &i18n!("enabled"),
                    set_subtitle: &i18n!("fps-unlocker-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        "90",
                        "120",
                        "144",
                        "165",
                        "180",
                        "200",
                        "240",
                        &i18n!("custom")
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

                    connect_selected_notify => |row| {
                        if is_ready()
                            && row.selected() < Fps::list().len() as u32 - 1
                            && let Ok(mut config) = Config::get()
                        {
                            config.game.enhancements.fps_unlocker.config.fps = Fps::list()[row.selected() as usize].to_num();

                            Config::update(config);
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_active: CONFIG.game.enhancements.fps_unlocker.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.fps_unlocker.enabled = switch.is_active();

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("fps-unlocker-interval"),
                    set_subtitle: &i18n!("fps-unlocker-interval-description"),

                    add_suffix = &gtk::SpinButton {
                        set_valign: gtk::Align::Center,
                        set_adjustment: &gtk::Adjustment::new(1.0, 1000.0, 60000.0, 1000.0, 1.0, 0.0),

                        set_value: CONFIG.game.enhancements.fps_unlocker.config.interval as f64,

                        connect_changed => |row| {
                            if is_ready() && let Ok(mut config) = Config::get() {
                                config.game.enhancements.fps_unlocker.config.interval = row.value() as u64;

                                Config::update(config);
                            }
                        }
                    }
                },
            }
        },

        #[local_ref]
        game_page -> adw::NavigationPage,

        #[local_ref]
        sandbox_page -> adw::NavigationPage,

        #[local_ref]
        environment_page -> adw::NavigationPage,
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing enhancements settings");

        let model = Self {
            gamescope: GamescopeApp::builder()
                .launch(())
                .detach(),

            game_page: GamePage::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            sandbox_page: SandboxPage::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            environment_page: EnvironmentPage::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            timeout_fix: CONFIG.game.wine.timeout_fix
        };

        let game_page = model.game_page.widget();
        let sandbox_page = model.sandbox_page.widget();
        let environment_page = model.environment_page.widget();

        let widgets = view_output!();

        if !is_wayland_available() {
            widgets.winewayland_row.set_tooltip_text(Some(&i18n!("winewayland-unavailable-tooltip")));
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        // Don't care about it, don't want to rewrite everything.
        #[allow(static_mut_refs)]
        match msg {
            EnhancementsAppMsg::SetGamescopeParent => unsafe {
                self.gamescope.widget().set_transient_for(super::main::PREFERENCES_WINDOW.as_ref());
            }

            EnhancementsAppMsg::OpenGamescope => {
                self.gamescope.widget().present();
            }

            EnhancementsAppMsg::OpenMainPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .pop_subpage();
            }

            EnhancementsAppMsg::OpenGameSettingsPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .push_subpage(self.game_page.widget());
            }

            EnhancementsAppMsg::OpenSandboxSettingsPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .push_subpage(self.sandbox_page.widget());
            }

            EnhancementsAppMsg::OpenEnvironmentSettingsPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .push_subpage(self.environment_page.widget());
            }

            EnhancementsAppMsg::SetTimeoutFix(value) => {
                self.timeout_fix = value;
            }

            EnhancementsAppMsg::Toast { title, description } => {
                let _ = sender.output(PreferencesAppMsg::Toast {
                    title,
                    description
                });
            }
        }
    }
}
