use relm4::prelude::*;
use relm4::factory::{AsyncFactoryComponent, AsyncFactorySender, AsyncFactoryVecDeque};
use gtk::prelude::*;
use adw::prelude::*;
use anime_launcher_sdk::wincompatlib::prelude::*;
use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;
use anime_launcher_sdk::genshin::config::schema::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::consts::GameEdition;
use anime_launcher_sdk::genshin::env_emulation::Environment;

pub mod components;

use components::*;

use crate::i18n::*;
use crate::*;
use super::main::PreferencesAppMsg;

/// Game environment options exposed in the general settings
///
/// Order of items defines their indices in the `game-environment` combo row
const ENVIRONMENTS: &[Environment] = &[
    Environment::PC,
    Environment::Android,
    Environment::Bilibili
];

/// Labels of the `game-environment` combo row
///
/// Must be in the same order as `ENVIRONMENTS`
const ENVIRONMENT_LABELS: &[&str] = &[
    "Hoyoverse",
    "Google Play",
    "Bilibili"
];

/// State of the Bilibili channel server plugin
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BilibiliPluginState {
    NotInstalled,

    Downloading {
        downloaded: u64,
        total: u64
    },

    Installed
}

impl BilibiliPluginState {
    /// Check if the plugin is being downloaded right now
    #[inline]
    fn is_downloading(&self) -> bool {
        matches!(self, Self::Downloading { .. })
    }

    /// Get plugin downloading progress in 0.0..=1.0 range
    #[inline]
    fn fraction(&self) -> f64 {
        match self {
            Self::Downloading { downloaded, total } if *total > 0 => {
                *downloaded as f64 / *total as f64
            }

            _ => 0.0
        }
    }
}

#[derive(Debug)]
struct VoicePackageComponent {
    locale: VoiceLocale,
    installed: bool,
    sensitive: bool
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for VoicePackageComponent {
    type CommandOutput = ();
    type Init = (VoiceLocale, bool);
    type Input = GeneralAppMsg;
    type Output = GeneralAppMsg;
    type ParentWidget = adw::ExpanderRow;

    view! {
        root = adw::ActionRow {
            set_title: &tr!(&self.locale.to_name().to_ascii_lowercase()),

            add_suffix = &gtk::Button {
                #[watch]
                set_visible: self.installed,

                #[watch]
                set_sensitive: self.sensitive,

                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.input(GeneralAppMsg::RemoveVoicePackage(index.clone()));
                }
            },

            add_suffix = &gtk::Button {
                #[watch]
                set_visible: !self.installed,

                #[watch]
                set_sensitive: self.sensitive,

                set_icon_name: "document-save-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.input(GeneralAppMsg::AddVoicePackage(index.clone()));
                }
            }
        }
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>
    ) -> Self {
        Self {
            locale: init.0,
            installed: init.1,
            sensitive: true
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        self.installed = !self.installed;

        sender.output(msg).unwrap();
    }
}

pub struct GeneralApp {
    voice_packages: AsyncFactoryVecDeque<VoicePackageComponent>,
    components_page: AsyncController<ComponentsPage>,

    game_diff: Option<Box<VersionDiff>>,
    style: LauncherStyle,
    environment: Environment,
    bilibili_plugin: BilibiliPluginState,
    use_video_background: bool,
    background_index: u8,
    languages: Vec<String>
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum GeneralAppMsg {
    /// Supposed to be called automatically on app's run when the latest game
    /// version was retrieved from the API
    SetGameDiff(Option<Box<VersionDiff>>),

    // If one ever wish to change it to accept VoiceLocale
    // I'd recommend to use clone!(@strong self.locale as locale => move |_| { .. })
    // in the VoicePackage component
    AddVoicePackage(DynamicIndex),
    RemoveVoicePackage(DynamicIndex),
    SetVoicePackageSensitivity(DynamicIndex, bool),

    UpdateDownloadedWine,
    UpdateDownloadedDxvk,

    RepairGame,
    RemakePrefix,

    OpenMainPage,
    OpenComponentsPage,

    UpdateLauncherStyle(LauncherStyle),
    SetVideoBackground(bool),

    SetEnvironment(Environment),
    InstallBilibiliPlugin,
    UpdateBilibiliPluginState,
    BilibiliPluginProgress(u64, u64),
    BilibiliPluginInstalled,
    BilibiliPluginNotInstalled,

    WineOpen(&'static [&'static str]),

    Toast {
        title: String,
        description: Option<String>
    }
}

impl GeneralApp {
    /// Get launcher edition from the config
    ///
    /// Unlike `CONFIG` it always contains the up-to-date value
    fn bilibili_plugin_edition() -> GameEdition {
        Config::get()
            .map(|config| config.launcher.edition)
            .unwrap_or(CONFIG.launcher.edition)
    }

    /// Check if the Bilibili plugin can be installed for the current game edition
    #[inline]
    fn bilibili_plugin_installable(&self) -> bool {
        Self::bilibili_plugin_edition() == GameEdition::China
    }

    /// Get Bilibili plugin settings row subtitle
    fn bilibili_plugin_status(&self) -> String {
        if !self.bilibili_plugin_installable() {
            return tr!("bilibili-plugin-china-only");
        }

        match self.bilibili_plugin {
            BilibiliPluginState::Installed => tr!("bilibili-plugin-installed"),
            BilibiliPluginState::Downloading { .. } => tr!("downloading"),
            BilibiliPluginState::NotInstalled => tr!("bilibili-plugin-description")
        }
    }

    /// Update Bilibili plugin installation state from the game folder
    fn update_bilibili_plugin_state(&mut self) {
        // Don't interrupt the installation
        if self.bilibili_plugin.is_downloading() {
            return;
        }

        self.bilibili_plugin = match Config::get() {
            Ok(config) => {
                let game_path = config.game.path.for_edition(config.launcher.edition);

                if anime_launcher_sdk::genshin::env_emulation::is_bilibili_plugin_installed(game_path) {
                    BilibiliPluginState::Installed
                }

                else {
                    BilibiliPluginState::NotInstalled
                }
            }

            Err(_) => BilibiliPluginState::NotInstalled
        };
    }

    /// Download and install Bilibili plugin in a separate thread
    fn start_bilibili_plugin_install(&mut self, sender: AsyncComponentSender<Self>) {
        if self.bilibili_plugin.is_downloading() {
            return;
        }

        let Ok(config) = Config::get()
        else {
            return;
        };

        if config.launcher.edition != GameEdition::China {
            return;
        }

        let game_path = config
            .game
            .path
            .for_edition(config.launcher.edition)
            .to_path_buf();

        self.bilibili_plugin = BilibiliPluginState::Downloading {
            downloaded: 0,
            total: 0
        };

        std::thread::spawn(move || {
            let progress_sender = sender.clone();

            let result = anime_launcher_sdk::genshin::env_emulation::install_bilibili_plugin(
                game_path,
                move |downloaded, total| {
                    let _ = progress_sender.input(
                        GeneralAppMsg::BilibiliPluginProgress(downloaded, total)
                    );
                }
            );

            match result {
                Ok(()) => sender.input(GeneralAppMsg::BilibiliPluginInstalled),

                Err(err) => {
                    tracing::error!("Failed to install Bilibili plugin: {err}");

                    sender.input(GeneralAppMsg::BilibiliPluginNotInstalled);

                    sender.input(GeneralAppMsg::Toast {
                        title: tr!("downloading-failed"),
                        description: Some(err.to_string())
                    });
                }
            }
        });
    }
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GeneralApp {
    type Init = ();
    type Input = GeneralAppMsg;
    type Output = PreferencesAppMsg;

    view! {
        #[root]
        adw::PreferencesPage {
            set_title: &tr!("general"),
            set_icon_name: Some("applications-system-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr!("appearance"),

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

                            gtk::Picture {
                                set_resource: Some(&format!("{APP_RESOURCE_PATH}/images/modern.svg")),
                                set_content_fit: gtk::ContentFit::ScaleDown
                            },

                            connect_clicked => GeneralAppMsg::UpdateLauncherStyle(LauncherStyle::Modern)
                        },

                        gtk::Label {
                            set_text: &tr!("modern"),

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

                            gtk::Picture {
                                set_resource: Some(&format!("{APP_RESOURCE_PATH}/images/classic.svg")),
                                set_content_fit: gtk::ContentFit::ScaleDown
                            },

                            connect_clicked => GeneralAppMsg::UpdateLauncherStyle(LauncherStyle::Classic)
                        },

                        gtk::Label {
                            set_text: &tr!("classic"),

                            set_margin_top: 16
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                #[watch]
                set_visible: model.style == LauncherStyle::Classic,

                adw::ActionRow {
                    set_title: &tr!("update-background"),
                    set_subtitle: &tr!("update-background-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_active: !KEEP_BACKGROUND_FILE.exists(),

                        connect_state_notify => |switch| {
                            #[allow(unused_must_use)]
                            if switch.is_active() {
                                std::fs::remove_file(KEEP_BACKGROUND_FILE.as_path());
                            } else {
                                std::fs::write(KEEP_BACKGROUND_FILE.as_path(), "");
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr!("video-background"),
                    set_subtitle: &tr!("video-background-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_active: !KEEP_BACKGROUND_FILE.exists(),

                        connect_state_notify[sender] => move |switch| {
                             sender.input(GeneralAppMsg::SetVideoBackground(switch.state()));
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr!("general"),

                adw::ComboRow {
                    set_title: &tr!("launcher-language"),
                    set_subtitle: &tr!("launcher-language-description"),

                    set_model: Some(&gtk::StringList::new(&model.languages.iter().map(|lang| lang.as_str()).collect::<Vec<&str>>())),

                    set_selected: {
                        let selected = crate::i18n::get_lang().language;

                        SUPPORTED_LANGUAGES.iter()
                            .position(|lang| lang.language == selected)
                            .unwrap_or(0) as u32
                    },

                    connect_selected_notify => |row| {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.launcher.language = crate::i18n::format_lang(SUPPORTED_LANGUAGES
                                    .get(row.selected() as usize)
                                    .unwrap_or(&SUPPORTED_LANGUAGES[0]));

                                Config::update(config);
                            }
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &tr!("game-edition"),

                    set_model: Some(&gtk::StringList::new(&[
                        &tr!("global"),
                        &tr!("china")
                    ])),

                    set_selected: GameEdition::list().iter()
                        .position(|edition| edition == &CONFIG.launcher.edition)
                        .unwrap() as u32,

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            #[allow(unused_must_use)]
                            if let Ok(mut config) = Config::get() {
                                config.launcher.edition = GameEdition::list()[row.selected() as usize];

                                Config::update(config);

                                // Bilibili plugin availability depends on the selected edition
                                sender.input(GeneralAppMsg::UpdateBilibiliPluginState);

                                sender.output(PreferencesAppMsg::UpdateLauncherState);
                            }
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &tr!("game-environment"),
                    set_subtitle: &tr!("game-environment-description"),

                    set_model: Some(&gtk::StringList::new(ENVIRONMENT_LABELS)),

                    set_selected: ENVIRONMENTS.iter()
                        .position(|environment| environment == &model.environment)
                        .unwrap_or(0) as u32,

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            if let Some(environment) = ENVIRONMENTS.get(row.selected() as usize) {
                                sender.input(GeneralAppMsg::SetEnvironment(*environment));
                            }
                        }
                    }
                },

                #[local_ref]
                voice_packages -> adw::ExpanderRow {
                    set_title: &tr!("game-voiceovers"),
                    set_subtitle: &tr!("game-voiceovers-description")
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_margin_top: 16,

                    gtk::Button {
                        set_label: &tr!("repair-game"),

                        connect_clicked => GeneralAppMsg::RepairGame
                    },

                    gtk::Button {
                        set_label: &tr!("remake-prefix"),

                        add_css_class: "destructive-action",

                        connect_clicked => GeneralAppMsg::RemakePrefix
                    }
                }
            },

            add = &adw::PreferencesGroup {
                #[watch]
                set_visible: model.environment == Environment::Bilibili,

                adw::ActionRow {
                    set_title: &tr!("bilibili-plugin"),

                    #[watch]
                    set_subtitle: &model.bilibili_plugin_status(),

                    add_suffix = &gtk::Button {
                        #[watch]
                        set_visible: !model.bilibili_plugin.is_downloading(),

                        #[watch]
                        set_sensitive: model.bilibili_plugin_installable(),

                        set_label: &tr!("download"),
                        set_valign: gtk::Align::Center,

                        connect_clicked => GeneralAppMsg::InstallBilibiliPlugin
                    },

                    add_suffix = &gtk::ProgressBar {
                        #[watch]
                        set_visible: model.bilibili_plugin.is_downloading(),

                        #[watch]
                        set_fraction: model.bilibili_plugin.fraction(),

                        set_valign: gtk::Align::Center,
                        set_width_request: 200
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr!("status"),

                adw::ActionRow {
                    set_title: &tr!("game-version"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_text: &match model.game_diff.as_ref() {
                            Some(diff) => match **diff {
                                VersionDiff::Latest { version: current, .. } |
                                VersionDiff::Predownload { current, .. } |
                                VersionDiff::Diff { current, .. } |
                                VersionDiff::Outdated { current, .. } => current.to_string(),

                                VersionDiff::NotInstalled { .. } => tr!("game-not-installed")
                            }

                            None => String::from("?")
                        },

                        #[watch]
                        set_css_classes: match model.game_diff.as_ref() {
                            Some(diff) => match **diff {
                                VersionDiff::Latest { .. }       => &["success"],
                                VersionDiff::Predownload { .. }  => &["accent"],
                                VersionDiff::Diff { .. }         => &["warning"],
                                VersionDiff::Outdated { .. }     => &["error"],
                                VersionDiff::NotInstalled { .. } => &[]
                            }

                            None => &[]
                        },

                        #[watch]
                        set_tooltip_text: Some(&match model.game_diff.as_ref() {
                            Some(diff) => match **diff {
                                VersionDiff::Latest { .. } => String::new(),

                                VersionDiff::Predownload { current, latest, .. } => tr!("game-predownload-available", {
                                    "old" = current.to_string(),
                                    "new" = latest.to_string()
                                }),

                                VersionDiff::Diff { current, latest, .. } => tr!("game-update-available", {
                                    "old" = current.to_string(),
                                    "new" = latest.to_string()
                                }),

                                VersionDiff::Outdated { latest, ..} => tr!("game-outdated", {
                                    "latest" = latest.to_string()
                                }),

                                VersionDiff::NotInstalled { .. } => String::new()
                            }

                            None => String::new()
                        })
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr!("options"),

                adw::ComboRow {
                    set_title: &tr!("launcher-behavior"),
                    set_subtitle: &tr!("launcher-behavior-description"),

                    set_model: Some(&gtk::StringList::new(&[
                        &tr!("nothing"),
                        &tr!("hide", { "form" = "verb" }),
                        &tr!("close", { "form" = "verb" }),
                    ])),

                    set_selected: match CONFIG.launcher.behavior {
                        LauncherBehavior::Nothing => 0,
                        LauncherBehavior::Hide    => 1,
                        LauncherBehavior::Close   => 2
                    },

                    connect_selected_notify => |row| {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.launcher.behavior = [
                                    LauncherBehavior::Nothing,
                                    LauncherBehavior::Hide,
                                    LauncherBehavior::Close
                                ][row.selected() as usize];

                                Config::update(config);
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: &tr!("components"),
                    set_subtitle: &tr!("components-description"),

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => GeneralAppMsg::OpenComponentsPage
                },

                adw::ExpanderRow {
                    set_title: &tr!("wine-tools"),

                    add_row = &adw::ActionRow {
                        set_title: &tr!("command-line"),
                        set_subtitle: "wineconsole",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["wineconsole"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr!("registry-editor"),
                        set_subtitle: "regedit",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["regedit"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr!("explorer"),
                        set_subtitle: "explorer",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["explorer"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr!("task-manager"),
                        set_subtitle: "taskmgr",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["taskmgr"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr!("configuration"),
                        set_subtitle: "winecfg",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["winecfg"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr!("debugger"),
                        set_subtitle: "start winedbg",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["start", "winedbg"])
                    }
                }
            }
        },

        #[local_ref]
        components_page -> adw::NavigationPage,
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing general settings");

        let mut model = Self {
            voice_packages: AsyncFactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), std::convert::identity),

            components_page: ComponentsPage::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            game_diff: None,
            style: CONFIG.launcher.style,
            environment: CONFIG.launcher.environment,
            bilibili_plugin: BilibiliPluginState::NotInstalled,
            use_video_background: CONFIG.launcher.video_background,
            background_index: CONFIG.launcher.background_index,
            languages: SUPPORTED_LANGUAGES
                .iter()
                .map(|lang| tr!(format_lang(lang).as_str()))
                .collect()
        };

        model.update_bilibili_plugin_state();

        for package in VoiceLocale::list() {
            model.voice_packages.guard().push_back((
                *package,
                CONFIG
                    .game
                    .voices
                    .iter()
                    .any(|voice| VoiceLocale::from_str(voice) == Some(*package))
            ));
        }

        let voice_packages = model.voice_packages.widget();
        let components_page = model.components_page.widget();

        let widgets = view_output!();

        AsyncComponentParts {
            model,
            widgets
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        tracing::debug!("Called general settings event: {:?}", msg);

        match msg {
            GeneralAppMsg::SetGameDiff(diff) => {
                self.game_diff = diff;
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::AddVoicePackage(index) => {
                if let Some(package) = self.voice_packages.get(index.current_index()) {
                    if let Ok(mut config) = Config::get() {
                        if !config
                            .game
                            .voices
                            .iter()
                            .any(|voice| VoiceLocale::from_str(voice) == Some(package.locale))
                        {
                            config
                                .game
                                .voices
                                .push(package.locale.to_code().to_string());

                            Config::update(config);

                            sender.output(PreferencesAppMsg::UpdateLauncherState);
                        }
                    }
                }
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::RemoveVoicePackage(index) => {
                if let Some(package) = self.voice_packages.guard().get_mut(index.current_index()) {
                    if let Ok(mut config) = Config::get() {
                        package.sensitive = false;

                        config
                            .game
                            .voices
                            .retain(|voice| VoiceLocale::from_str(voice) != Some(package.locale));

                        Config::update(config.clone());

                        let package =
                            VoicePackage::with_locale(package.locale, config.launcher.edition)
                                .unwrap();
                        let game_path = config
                            .game
                            .path
                            .for_edition(config.launcher.edition)
                            .to_path_buf();

                        if package.is_installed_in(&game_path) {
                            std::thread::spawn(move || {
                                if let Err(err) = package.delete_in(game_path) {
                                    tracing::error!(
                                        "Failed to delete voice package: {:?}",
                                        package.locale()
                                    );

                                    sender.input(GeneralAppMsg::Toast {
                                        title: tr!("voice-package-deletion-error"),
                                        description: Some(err.to_string())
                                    });
                                }

                                sender
                                    .input(GeneralAppMsg::SetVoicePackageSensitivity(index, true));
                                sender.output(PreferencesAppMsg::UpdateLauncherState);
                            });
                        } else {
                            sender.input(GeneralAppMsg::SetVoicePackageSensitivity(index, true));
                        }
                    }
                }
            }

            GeneralAppMsg::SetVoicePackageSensitivity(index, sensitive) => {
                if let Some(package) = self.voice_packages.guard().get_mut(index.current_index()) {
                    package.sensitive = sensitive;
                }
            }

            GeneralAppMsg::UpdateDownloadedWine => {
                self.components_page
                    .sender()
                    .send(ComponentsPageMsg::UpdateDownloadedWine)
                    .unwrap();
            }

            GeneralAppMsg::UpdateDownloadedDxvk => {
                self.components_page
                    .sender()
                    .send(ComponentsPageMsg::UpdateDownloadedDxvk)
                    .unwrap();
            }

            GeneralAppMsg::RepairGame => {
                sender.output(Self::Output::RepairGame).unwrap();
            }

            GeneralAppMsg::RemakePrefix => {
                sender.output(Self::Output::RemakePrefix).unwrap();
            }

            // Don't care about it, don't want to rewrite everything.
            #[allow(static_mut_refs)]
            GeneralAppMsg::OpenMainPage => unsafe {
                PREFERENCES_WINDOW
                    .as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .pop_subpage();
            },

            // Don't care about it, don't want to rewrite everything.
            #[allow(static_mut_refs)]
            GeneralAppMsg::OpenComponentsPage => unsafe {
                PREFERENCES_WINDOW
                    .as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .push_subpage(self.components_page.widget());
            },

            GeneralAppMsg::UpdateLauncherStyle(style) => {
                if style == LauncherStyle::Classic && !KEEP_BACKGROUND_FILE.exists() {
                    if let Err(err) = crate::background::download_background(
                        self.use_video_background,
                        self.background_index
                    ) {
                        tracing::error!("Failed to download background picture/video");

                        sender.input(GeneralAppMsg::Toast {
                            title: tr!("background-downloading-failed"),
                            description: Some(err.to_string())
                        });

                        return;
                    }
                }

                if let Ok(mut config) = Config::get() {
                    config.launcher.style = style;

                    Config::update(config);
                }

                self.style = style;

                let _ = sender.output(Self::Output::SetLauncherStyle(style));
            }

            GeneralAppMsg::SetVideoBackground(use_video) => {
                if self.style == LauncherStyle::Classic
                    && !KEEP_BACKGROUND_FILE.exists()
                    && use_video
                {
                    if let Err(err) = crate::background::download_background(
                        self.use_video_background,
                        self.background_index
                    ) {
                        tracing::error!("Failed to download background picture/video");

                        sender.input(GeneralAppMsg::Toast {
                            title: tr!("background-downloading-failed"),
                            description: Some(err.to_string())
                        });

                        return;
                    }
                }

                if let Ok(mut config) = Config::get() {
                    config.launcher.video_background = use_video;

                    Config::update(config);
                }

                self.use_video_background = use_video;

                let _ = sender.output(Self::Output::SetVideoBackground(use_video));
            }

            GeneralAppMsg::SetEnvironment(environment) => {
                if let Ok(mut config) = Config::get() {
                    config.launcher.environment = environment;

                    Config::update(config);
                }

                self.environment = environment;

                self.update_bilibili_plugin_state();

                // Bilibili plugin is required for the game to actually connect
                // to the Bilibili channel server, so we're installing it right away
                if environment == Environment::Bilibili
                    && self.bilibili_plugin == BilibiliPluginState::NotInstalled
                {
                    self.start_bilibili_plugin_install(sender);
                }
            }

            GeneralAppMsg::InstallBilibiliPlugin => {
                self.start_bilibili_plugin_install(sender);
            }

            GeneralAppMsg::UpdateBilibiliPluginState => {
                self.update_bilibili_plugin_state();
            }

            GeneralAppMsg::BilibiliPluginProgress(downloaded, total) => {
                self.bilibili_plugin = BilibiliPluginState::Downloading {
                    downloaded,
                    total
                };
            }

            GeneralAppMsg::BilibiliPluginInstalled => {
                self.bilibili_plugin = BilibiliPluginState::Installed;
            }

            GeneralAppMsg::BilibiliPluginNotInstalled => {
                self.bilibili_plugin = BilibiliPluginState::NotInstalled;
            }

            GeneralAppMsg::WineOpen(executable) => {
                let config = Config::get().unwrap_or_else(|_| CONFIG.clone());

                if let Ok(Some(wine)) = config.get_selected_wine() {
                    let result = wine
                        .to_wine(
                            config.components.path,
                            Some(config.game.wine.builds.join(&wine.name))
                        )
                        .with_prefix(config.game.wine.prefix)
                        .with_loader(WineLoader::Current)
                        .with_arch(WineArch::Win64)
                        .run_args(executable);

                    if let Err(err) = result {
                        sender.input(GeneralAppMsg::Toast {
                            title: tr!("wine-run-error", { "executable" = executable.join(" ") }),
                            description: Some(err.to_string())
                        });

                        tracing::error!("Failed to run {:?} using wine: {err}", executable);
                    }
                }
            }

            GeneralAppMsg::Toast {
                title,
                description
            } => {
                let _ = sender.output(Self::Output::Toast {
                    title,
                    description
                });
            }
        }
    }
}
