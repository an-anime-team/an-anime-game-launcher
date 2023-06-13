use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;
use anime_launcher_sdk::config::schema_blanks::prelude::*;

use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;

use anime_launcher_sdk::discord_rpc::DiscordRpc;
use anime_launcher_sdk::is_available;

pub mod game;
pub mod sandbox;
pub mod environment;

use game::*;
use sandbox::*;
use environment::*;

use crate::i18n::tr;
use crate::*;

use super::gamescope::*;
use super::main::PreferencesAppMsg;

#[derive(Debug)]
struct DiscordRpcIcon {
    pub check_button: gtk::CheckButton,

    pub name: String,
    pub path: PathBuf
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for DiscordRpcIcon {
    type Init = Self;
    type Input = EnhancementsAppMsg;
    type Output = EnhancementsAppMsg;
    type CommandOutput = ();
    type ParentInput = EnhancementsAppMsg;
    type ParentWidget = adw::ExpanderRow;

    view! {
        root = adw::ActionRow {
            set_title: &self.name,
            // set_subtitle: &self.name,

            // Don't even try to understand
            add_prefix = &self.check_button.clone(),

            add_suffix = &gtk::Picture {
                set_margin_start: 4,
                set_margin_top: 4,
                set_margin_end: 4,
                set_margin_bottom: 4,

                add_css_class: "round-bin",

                set_filename: Some(&self.path)
            },

            set_activatable: true,

            connect_activated[sender, index] => move |_| {
                sender.output(EnhancementsAppMsg::SetDiscordRpcIcon(index.clone()));
            }
        }
    }

    #[inline]
    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        init
    }

    #[inline]
    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }
}

pub struct EnhancementsApp {
    discord_rpc_icons: AsyncFactoryVecDeque<DiscordRpcIcon>,
    discord_rpc_root_check_button: gtk::CheckButton,

    gamescope: AsyncController<GamescopeApp>,
    game_page: AsyncController<GamePage>,
    sandbox_page: AsyncController<SandboxPage>,
    environment_page: AsyncController<EnvironmentPage>
}

#[derive(Debug)]
pub enum EnhancementsAppMsg {
    SetGamescopeParent(adw::PreferencesWindow),

    SetDiscordRpcIcon(DynamicIndex),

    OpenGamescope,
    OpenMainPage,
    OpenGameSettingsPage,
    OpenSandboxSettingsPage,
    OpenEnvironmentSettingsPage,

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
            set_title: &tr("enhancements"),
            set_icon_name: Some("applications-graphics-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr("options"),

                adw::ActionRow {
                    set_title: &tr("game"),
                    set_subtitle: &tr("game-settings-description"),

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => EnhancementsAppMsg::OpenGameSettingsPage
                },

                adw::ActionRow {
                    set_title: &tr("sandbox"),
                    set_subtitle: &tr("sandbox-settings-description"),

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => EnhancementsAppMsg::OpenSandboxSettingsPage
                },

                adw::ActionRow {
                    set_title: &tr("environment"),
                    set_subtitle: &tr("environment-settings-description"),

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => EnhancementsAppMsg::OpenEnvironmentSettingsPage
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("wine"),

                adw::ComboRow {
                    set_title: &tr("synchronization"),
                    set_subtitle: &tr("wine-sync-description"),

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        &tr("none"),
                        "ESync",
                        "FSync"
                    ]),

                    set_selected: CONFIG.game.wine.sync.ordinal() as u32,

                    connect_selected_notify => |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.game.wine.sync = WineSync::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
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

                    connect_selected_notify => |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.game.wine.language = WineLang::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr("borderless-window"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.wine.borderless,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.wine.borderless = switch.state();

                                    Config::update(config);
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

                    connect_selected_notify => |row| {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                let (width, height) = Resolution::try_from(row.selected()).unwrap().get_pair();

                                config.game.wine.virtual_desktop.width = width;
                                config.game.wine.virtual_desktop.height = height;

                                Config::update(config);
                            }
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.wine.virtual_desktop.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.wine.virtual_desktop.enabled = switch.state();

                                    Config::update(config);
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

                    connect_selected_notify => |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.game.enhancements.hud = HUD::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
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

                    set_selected: CONFIG.game.enhancements.fsr.quality.ordinal() as u32,

                    connect_selected_notify => |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.game.enhancements.fsr.quality = FsrQuality::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
                            }
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.fsr.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.fsr.enabled = switch.state();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr("gamemode"),
                    set_subtitle: &tr("gamemode-description"),

                    set_sensitive: is_available("gamemoderun"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.gamemode,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamemode = switch.state();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr("gamescope"),
                    set_subtitle: &tr("gamescope-description"),

                    set_sensitive: is_available("gamescope"),

                    add_suffix = &gtk::Button {
                        set_icon_name: "emblem-system-symbolic",
                        add_css_class: "flat",

                        set_valign: gtk::Align::Center,

                        connect_clicked => EnhancementsAppMsg::OpenGamescope
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.gamescope.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.gamescope.enabled = switch.state();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("discord-rpc"),

                adw::ActionRow {
                    set_title: &tr("enabled"),
                    set_subtitle: &tr("discord-rpc-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_state: CONFIG.launcher.discord_rpc.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.launcher.discord_rpc.enabled = switch.state();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                #[local_ref]
                discord_rpc_icons -> adw::ExpanderRow {
                    set_title: &tr("icon")
                },

                adw::EntryRow {
                    set_title: &tr("title"),
                    set_text: &CONFIG.launcher.discord_rpc.title,

                    connect_changed: |row| {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.launcher.discord_rpc.title = row.text().to_string();

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::EntryRow {
                    set_title: &tr("description"),
                    set_text: &CONFIG.launcher.discord_rpc.subtitle,

                    connect_changed: |row| {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.launcher.discord_rpc.subtitle = row.text().to_string();

                                Config::update(config);
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

                    connect_selected_notify => |row| {
                        if is_ready() && row.selected() < Fps::list().len() as u32 - 1 {
                            if let Ok(mut config) = Config::get() {
                                config.game.enhancements.fps_unlocker.config.fps = Fps::list()[row.selected() as usize].to_num();

                                Config::update(config);
                            }
                        }
                    },

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.game.enhancements.fps_unlocker.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.fps_unlocker.enabled = switch.state();

                                    Config::update(config);
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

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.fps_unlocker.config.power_saving = switch.state();

                                    Config::update(config);
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

                        connect_changed => |row| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.game.enhancements.fps_unlocker.config.monitor = row.value() as u64;

                                    Config::update(config);
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

                    connect_selected_notify => |row| unsafe {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.game.enhancements.fps_unlocker.config.window_mode = WindowMode::from_ordinal_unsafe(row.selected() as i8);

                                Config::update(config);
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

                    connect_selected_notify => |row| {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.game.enhancements.fps_unlocker.config.priority = row.selected() as u64;

                                Config::update(config);
                            }
                        }
                    }
                },
            }
        },

        #[local_ref]
        game_page -> gtk::Box {},

        #[local_ref]
        sandbox_page -> gtk::Box {},

        #[local_ref]
        environment_page -> gtk::Box {}
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing enhancements settings");

        let mut model = Self {
            discord_rpc_icons: AsyncFactoryVecDeque::new(adw::ExpanderRow::new(), sender.input_sender()),
            discord_rpc_root_check_button: gtk::CheckButton::new(),

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
                .forward(sender.input_sender(), std::convert::identity)
        };

        match DiscordRpc::get_assets(CONFIG.launcher.discord_rpc.app_id) {
            Ok(icons) => {
                for icon in icons {
                    let cache_file = CACHE_FOLDER.join("discord-rpc").join(&icon.name);
                    // let sender = sender.clone();

                    if !cache_file.exists() {
                        std::thread::spawn(move || {
                            Downloader::new(icon.get_uri())
                                .expect("Failed to init Discord RPC icon downloader")
                                .with_continue_downloading(false)
                                .with_free_space_check(false)
                                .download(cache_file, |_, _| {})
                                .expect("Failed to download Discord RPC icon");

                            /*if let Err(err) = result {
                                sender.input(EnhancementsAppMsg::Toast {
                                    title: tr("discord-rpc-icon-download-failed"),
                                    description: Some(err.to_string())
                                });
                            }*/
                        });
                    }

                    // TODO: add icons after thread above finishes its work as well
                    else {
                        let check_button = gtk::CheckButton::new();

                        check_button.set_group(Some(&model.discord_rpc_root_check_button));

                        if CONFIG.launcher.discord_rpc.icon == icon.name {
                            check_button.set_active(true);
                        }

                        model.discord_rpc_icons.guard().push_back(DiscordRpcIcon {
                            check_button,
                            name: icon.name.clone(),
                            path: cache_file.clone()
                        });
                    }
                }
            }

            Err(err) => sender.input(EnhancementsAppMsg::Toast {
                title: tr("discord-rpc-icons-fetch-failed"),
                description: Some(err.to_string())
            })
        }

        let discord_rpc_icons = model.discord_rpc_icons.widget();

        let game_page = model.game_page.widget();
        let sandbox_page = model.sandbox_page.widget();
        let environment_page = model.environment_page.widget();

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            EnhancementsAppMsg::SetGamescopeParent(parent) => {
                self.gamescope.widget().set_transient_for(Some(&parent));
            }

            EnhancementsAppMsg::SetDiscordRpcIcon(index) => {
                if let Some(icon) = self.discord_rpc_icons.guard().get(index.current_index()) {
                    if let Ok(mut config) = Config::get() {
                        config.launcher.discord_rpc.icon = icon.name.clone();

                        Config::update(config);

                        icon.check_button.set_active(true);
                    }
                }
            }

            EnhancementsAppMsg::OpenGamescope => {
                self.gamescope.widget().present();
            }

            EnhancementsAppMsg::OpenMainPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .close_subpage();
            }

            EnhancementsAppMsg::OpenGameSettingsPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .present_subpage(self.game_page.widget());
            }

            EnhancementsAppMsg::OpenSandboxSettingsPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .present_subpage(self.sandbox_page.widget());
            }

            EnhancementsAppMsg::OpenEnvironmentSettingsPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .present_subpage(self.environment_page.widget());
            }

            EnhancementsAppMsg::Toast { title, description } => {
                sender.output(PreferencesAppMsg::Toast {
                    title,
                    description
                }).unwrap();
            }
        }
    }
}
