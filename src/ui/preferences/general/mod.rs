use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::{
    AsyncFactoryVecDeque,
    AsyncFactoryComponent,
    AsyncFactorySender
};

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::wincompatlib::prelude::*;

use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::consts::GameEdition;

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;

use anime_launcher_sdk::genshin::config::schema::launcher::LauncherStyle;
use anime_launcher_sdk::genshin::env_emulation::Environment;

pub mod components;

use components::*;
use super::main::PreferencesAppMsg;

use crate::ui::migrate_installation::MigrateInstallationApp;
use crate::i18n::*;
use crate::*;

#[derive(Debug)]
struct VoicePackageComponent {
    locale: VoiceLocale,
    installed: bool,
    sensitive: bool
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for VoicePackageComponent {
    type Init = (VoiceLocale, bool);
    type Input = GeneralAppMsg;
    type Output = GeneralAppMsg;
    type CommandOutput = ();
    type ParentInput = GeneralAppMsg;
    type ParentWidget = adw::ExpanderRow;

    view! {
        root = adw::ActionRow {
            set_title: &tr(&self.locale.to_name().to_ascii_lowercase()),

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
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            locale: init.0,
            installed: init.1,
            sensitive: true
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        self.installed = !self.installed;

        sender.output(msg);
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }
}

pub struct GeneralApp {
    voice_packages: AsyncFactoryVecDeque<VoicePackageComponent>,
    migrate_installation: Controller<MigrateInstallationApp>,
    components_page: AsyncController<ComponentsPage>,

    game_diff: Option<VersionDiff>,
    unity_player_patch: Option<UnityPlayerPatch>,
    xlua_patch: Option<XluaPatch>,

    style: LauncherStyle,

    languages: Vec<String>
}

#[derive(Debug, Clone)]
pub enum GeneralAppMsg {
    /// Supposed to be called automatically on app's run when the latest game version
    /// was retrieved from the API
    SetGameDiff(Option<VersionDiff>),

    /// Supposed to be called automatically on app's run when the latest UnityPlayer patch version
    /// was retrieved from remote repos
    SetUnityPlayerPatch(Option<UnityPlayerPatch>),

    /// Supposed to be called automatically on app's run when the latest xlua patch version
    /// was retrieved from remote repos
    SetXluaPatch(Option<XluaPatch>),

    // If one ever wish to change it to accept VoiceLocale
    // I'd recommend to use clone!(@strong self.locale as locale => move |_| { .. })
    // in the VoicePackage component
    AddVoicePackage(DynamicIndex),
    RemoveVoicePackage(DynamicIndex),
    SetVoicePackageSensitivity(DynamicIndex, bool),

    UpdateDownloadedWine,
    UpdateDownloadedDxvk,

    OpenMigrateInstallation,
    RepairGame,

    OpenMainPage,
    OpenComponentsPage,

    UpdateLauncherStyle(LauncherStyle),

    WineOpen(&'static [&'static str]),

    Toast {
        title: String,
        description: Option<String>
    }
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GeneralApp {
    type Init = ();
    type Input = GeneralAppMsg;
    type Output = PreferencesAppMsg;

    view! {
        #[root]
        main_page = adw::PreferencesPage {
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

                            connect_clicked => GeneralAppMsg::UpdateLauncherStyle(LauncherStyle::Modern)
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

                            connect_clicked => GeneralAppMsg::UpdateLauncherStyle(LauncherStyle::Classic)
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
                        set_valign: gtk::Align::Center,
                        set_active: !KEEP_BACKGROUND_FILE.exists(),

                        connect_state_notify => |switch| {
                            #[allow(unused_must_use)]
                            if switch.state() {
                                std::fs::remove_file(KEEP_BACKGROUND_FILE.as_path());
                            } else {
                                std::fs::write(KEEP_BACKGROUND_FILE.as_path(), "");
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("general"),

                adw::ComboRow {
                    set_title: &tr("launcher-language"),
                    set_subtitle: &tr("launcher-language-description"),

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
                    set_title: &tr("game-edition"),

                    set_model: Some(&gtk::StringList::new(&[
                        &tr("global"),
                        &tr("china")
                    ])),

                    set_selected: match CONFIG.launcher.edition {
                        GameEdition::Global => 0,
                        GameEdition::China => 1
                    },

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            #[allow(unused_must_use)]
                            if let Ok(mut config) = Config::get() {
                                config.launcher.edition = match row.selected() {
                                    0 => GameEdition::Global,
                                    1 => GameEdition::China,

                                    _ => unreachable!()
                                };

                                // Select new game edition
                                config.launcher.edition.select();

                                Config::update(config);

                                sender.output(PreferencesAppMsg::UpdateLauncherState);
                            }
                        }
                    }
                },

                adw::ComboRow {
                    set_title: &tr("game-environment"),
                    set_subtitle: &tr("game-environment-description"),

                    set_model: Some(&gtk::StringList::new(&[
                        "PC",
                        "Android"
                    ])),

                    set_selected: match CONFIG.launcher.environment {
                        Environment::PC => 0,
                        Environment::Android => 1,

                        _ => unreachable!()
                    },

                    connect_selected_notify => |row| {
                        if is_ready() {
                            if let Ok(mut config) = Config::get() {
                                config.launcher.environment = match row.selected() {
                                    0 => Environment::PC,
                                    1 => Environment::Android,

                                    _ => unreachable!()
                                };
    
                                Config::update(config);
                            }
                        }
                    }
                },

                #[local_ref]
                voice_packages -> adw::ExpanderRow {
                    set_title: &tr("game-voiceovers"),
                    set_subtitle: &tr("game-voiceovers-description")
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_margin_top: 16,

                    gtk::Button {
                        set_label: &tr("migrate-installation"),
                        set_tooltip_text: Some(&tr("migrate-installation-description")),

                        connect_clicked => GeneralAppMsg::OpenMigrateInstallation
                    },

                    gtk::Button {
                        set_label: &tr("repair-game"),

                        connect_clicked => GeneralAppMsg::RepairGame
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("status"),

                adw::ActionRow {
                    set_title: &tr("game-version"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_text: &match model.game_diff.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(current) |
                                VersionDiff::Predownload { current, .. } |
                                VersionDiff::Diff { current, .. } |
                                VersionDiff::Outdated { current, .. } => current.to_string(),

                                VersionDiff::NotInstalled { .. } => tr("game-not-installed")
                            }

                            None => String::from("?")
                        },

                        #[watch]
                        set_css_classes: match model.game_diff.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(_) => &["success"],
                                VersionDiff::Predownload { .. } => &["accent"],
                                VersionDiff::Diff { .. } => &["warning"],
                                VersionDiff::Outdated { .. } => &["error"],
                                VersionDiff::NotInstalled { .. } => &[]
                            }

                            None => &[]
                        },

                        #[watch]
                        set_tooltip_text: Some(&match model.game_diff.as_ref() {
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
                    set_title: &tr("player-patch-version"),
                    set_subtitle: &tr("player-patch-version-description"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_text: &match model.unity_player_patch.as_ref() {
                            Some(patch) => match patch.status() {
                                PatchStatus::NotAvailable => tr("patch-not-available"),
                                PatchStatus::Outdated { current, .. } => tr_args("patch-outdated", [("current", current.to_string().into())]),
                                PatchStatus::Preparation { .. } => tr("patch-preparation"),
                                PatchStatus::Testing { version, .. } |
                                PatchStatus::Available { version, .. } => version.to_string()
                            }

                            None => String::from("?")
                        },

                        #[watch]
                        set_css_classes: match model.unity_player_patch.as_ref() {
                            Some(patch) => match patch.status() {
                                PatchStatus::NotAvailable => &["error"],
                                PatchStatus::Outdated { .. } |
                                PatchStatus::Preparation { .. } |
                                PatchStatus::Testing { .. } => &["warning"],
                                PatchStatus::Available { .. } => unsafe {
                                    let path = match Config::get() {
                                        Ok(config) => config.game.path.for_edition(config.launcher.edition).to_path_buf(),
                                        Err(_) => CONFIG.game.path.for_edition(CONFIG.launcher.edition).to_path_buf(),
                                    };

                                    if let Ok(true) = model.unity_player_patch.as_ref().unwrap_unchecked().is_applied(path) {
                                        &["success"]
                                    } else {
                                        &["warning"]
                                    }
                                }
                            }

                            None => &[]
                        },

                        #[watch]
                        set_tooltip_text: Some(&match model.unity_player_patch.as_ref() {
                            Some(patch) => match patch.status() {
                                PatchStatus::NotAvailable => tr("patch-not-available-tooltip"),
                                PatchStatus::Outdated { current, latest, .. } => tr_args("patch-outdated-tooltip", [
                                    ("current", current.to_string().into()),
                                    ("latest", latest.to_string().into())
                                ]),
                                PatchStatus::Preparation { .. } => tr("patch-preparation-tooltip"),
                                PatchStatus::Testing { .. } => tr("patch-testing-tooltip"),
                                PatchStatus::Available { .. } => unsafe {
                                    let path = match Config::get() {
                                        Ok(config) => config.game.path.for_edition(config.launcher.edition).to_path_buf(),
                                        Err(_) => CONFIG.game.path.for_edition(CONFIG.launcher.edition).to_path_buf(),
                                    };

                                    if let Ok(true) = model.unity_player_patch.as_ref().unwrap_unchecked().is_applied(path) {
                                        String::new()
                                    } else {
                                        tr("patch-not-applied-tooltip")
                                    }
                                }
                            }

                            None => String::new()
                        })
                    }
                },

                adw::ActionRow {
                    set_title: &tr("xlua-patch-version"),
                    set_subtitle: &tr("xlua-patch-version-description"),

                    add_suffix = &gtk::Label {
                        #[watch]
                        set_text: &match model.xlua_patch.as_ref() {
                            Some(patch) => match patch.status() {
                                PatchStatus::NotAvailable => tr("patch-not-available"),
                                PatchStatus::Outdated { current, .. } => tr_args("patch-outdated", [("current", current.to_string().into())]),
                                PatchStatus::Preparation { .. } => tr("patch-preparation"),
                                PatchStatus::Testing { version, .. } |
                                PatchStatus::Available { version, .. } => version.to_string()
                            }

                            None => String::from("?")
                        },

                        #[watch]
                        set_css_classes: match model.xlua_patch.as_ref() {
                            Some(patch) => match patch.status() {
                                PatchStatus::NotAvailable => &["error"],
                                PatchStatus::Outdated { .. } |
                                PatchStatus::Preparation { .. } |
                                PatchStatus::Testing { .. } => &["warning"],
                                PatchStatus::Available { .. } => unsafe {
                                    let path = match Config::get() {
                                        Ok(config) => config.game.path.for_edition(config.launcher.edition).to_path_buf(),
                                        Err(_) => CONFIG.game.path.for_edition(CONFIG.launcher.edition).to_path_buf(),
                                    };

                                    if let Ok(true) = model.xlua_patch.as_ref().unwrap_unchecked().is_applied(path) {
                                        &["success"]
                                    } else {
                                        &["warning"]
                                    }
                                }
                            }

                            None => &[]
                        },

                        #[watch]
                        set_tooltip_text: Some(&match model.xlua_patch.as_ref() {
                            Some(patch) => match patch.status() {
                                PatchStatus::NotAvailable => tr("patch-not-available-tooltip"),
                                PatchStatus::Outdated { current, latest, .. } => tr_args("patch-outdated-tooltip", [
                                    ("current", current.to_string().into()),
                                    ("latest", latest.to_string().into())
                                ]),
                                PatchStatus::Preparation { .. } => tr("patch-preparation-tooltip"),
                                PatchStatus::Testing { .. } => tr("patch-testing-tooltip"),
                                PatchStatus::Available { .. } => unsafe {
                                    let path = match Config::get() {
                                        Ok(config) => config.game.path.for_edition(config.launcher.edition).to_path_buf(),
                                        Err(_) => CONFIG.game.path.for_edition(CONFIG.launcher.edition).to_path_buf(),
                                    };

                                    if let Ok(true) = model.xlua_patch.as_ref().unwrap_unchecked().is_applied(path) {
                                        String::new()
                                    } else {
                                        tr("patch-not-applied-tooltip")
                                    }
                                }
                            }

                            None => String::new()
                        })
                    }
                }
            },

            add = &adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: &tr("apply-xlua-patch"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.patch.apply_xlua,

                        connect_state_notify[sender] => move |switch| {
                            if is_ready() {
                                #[allow(unused_must_use)]
                                if let Ok(mut config) = Config::get() {
                                    config.patch.apply_xlua = switch.state();

                                    Config::update(config);

                                    sender.output(PreferencesAppMsg::UpdateLauncherState);
                                }
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: &tr("ask-superuser-permissions"),
                    set_subtitle: &tr("ask-superuser-permissions-description"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.patch.root,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.patch.root = switch.state();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: "Options",

                adw::ActionRow {
                    set_title: "Components",
                    set_subtitle: "Manage your Wine and DXVK versions",

                    add_suffix = &gtk::Image {
                        set_icon_name: Some("go-next-symbolic")
                    },

                    set_activatable: true,

                    connect_activated => GeneralAppMsg::OpenComponentsPage
                },

                adw::ExpanderRow {
                    set_title: &tr("wine-tools"),

                    add_row = &adw::ActionRow {
                        set_title: &tr("command-line"),
                        set_subtitle: "wineconsole",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["wineconsole"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("registry-editor"),
                        set_subtitle: "regedit",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["regedit"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("explorer"),
                        set_subtitle: "explorer",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["explorer"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("task-manager"),
                        set_subtitle: "taskmgr",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["taskmgr"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("configuration"),
                        set_subtitle: "winecfg",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["winecfg"])
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("debugger"),
                        set_subtitle: "start winedbg",

                        set_activatable: true,

                        connect_activated => GeneralAppMsg::WineOpen(&["start", "winedbg"])
                    }
                }
            }
        },

        #[local_ref]
        components_page -> gtk::Box {}
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing general settings");

        let mut model = Self {
            voice_packages: AsyncFactoryVecDeque::new(adw::ExpanderRow::new(), sender.input_sender()),

            migrate_installation: MigrateInstallationApp::builder()
                .launch(())
                .detach(),

            components_page: ComponentsPage::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            game_diff: None,
            unity_player_patch: None,
            xlua_patch: None,

            style: CONFIG.launcher.style,

            languages: SUPPORTED_LANGUAGES.iter().map(|lang| tr(format_lang(lang).as_str())).collect()
        };

        for package in VoiceLocale::list() {
            model.voice_packages.guard().push_back((
                *package,
                CONFIG.game.voices.iter().any(|voice| VoiceLocale::from_str(voice) == Some(*package))
            ));
        }

        let voice_packages = model.voice_packages.widget();
        let components_page = model.components_page.widget();

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        tracing::debug!("Called general settings event: {:?}", msg);

        match msg {
            GeneralAppMsg::SetGameDiff(diff) => {
                self.game_diff = diff;
            }

            GeneralAppMsg::SetUnityPlayerPatch(patch) => {
                self.unity_player_patch = patch;
            }

            GeneralAppMsg::SetXluaPatch(patch) => {
                self.xlua_patch = patch;
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::AddVoicePackage(index) => {
                if let Some(package) = self.voice_packages.get(index.current_index()) {
                    if let Ok(mut config) = Config::get() {
                        if !config.game.voices.iter().any(|voice| VoiceLocale::from_str(voice) == Some(package.locale)) {
                            config.game.voices.push(package.locale.to_code().to_string());

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

                        config.game.voices.retain(|voice| VoiceLocale::from_str(voice) != Some(package.locale));

                        Config::update(config.clone());

                        let package = VoicePackage::with_locale(package.locale).unwrap();
                        let game_path = config.game.path.for_edition(config.launcher.edition).to_path_buf();

                        if package.is_installed_in(&game_path) {
                            std::thread::spawn(move || {
                                if let Err(err) = package.delete_in(game_path) {
                                    tracing::error!("Failed to delete voice package: {:?}", package.locale());

                                    sender.input(GeneralAppMsg::Toast {
                                        title: tr("voice-package-deletion-error"),
                                        description: Some(err.to_string())
                                    });
                                }

                                sender.input(GeneralAppMsg::SetVoicePackageSensitivity(index, true));
                                sender.output(PreferencesAppMsg::UpdateLauncherState);
                            });
                        }

                        else {
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
                self.components_page.sender()
                    .send(ComponentsPageMsg::UpdateDownloadedWine)
                    .unwrap();
            }

            GeneralAppMsg::UpdateDownloadedDxvk => {
                self.components_page.sender()
                    .send(ComponentsPageMsg::UpdateDownloadedDxvk)
                    .unwrap();
            }

            GeneralAppMsg::OpenMigrateInstallation => unsafe {
                if let Some(window) = crate::ui::main::PREFERENCES_WINDOW.as_ref() {
                    self.migrate_installation.widget().set_transient_for(Some(window.widget()));
                }

                self.migrate_installation.widget().show();
            }

            GeneralAppMsg::RepairGame => {
                sender.output(Self::Output::RepairGame).unwrap();
            }

            GeneralAppMsg::OpenMainPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .close_subpage();
            }

            GeneralAppMsg::OpenComponentsPage => unsafe {
                PREFERENCES_WINDOW.as_ref()
                    .unwrap_unchecked()
                    .widget()
                    .present_subpage(self.components_page.widget());
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::UpdateLauncherStyle(style) => {
                if style == LauncherStyle::Classic && !KEEP_BACKGROUND_FILE.exists() {
                    if let Err(err) = crate::background::download_background() {
                        tracing::error!("Failed to download background picture");

                        sender.input(GeneralAppMsg::Toast {
                            title: tr("background-downloading-failed"),
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

                sender.output(Self::Output::SetLauncherStyle(style));
            }

            GeneralAppMsg::WineOpen(executable) => {
                let config = Config::get().unwrap_or_else(|_| CONFIG.clone());

                if let Ok(Some(wine)) = config.get_selected_wine() {
                    let result = wine
                        .to_wine(config.components.path, Some(config.game.wine.builds.join(&wine.name)))
                        .with_prefix(config.game.wine.prefix)
                        .with_loader(WineLoader::Current)
                        .with_arch(WineArch::Win64)
                        .run_args(executable);

                    if let Err(err) = result {
                        sender.input(GeneralAppMsg::Toast {
                            title: tr_args("wine-run-error", [
                                ("executable", executable.join(" ").into())
                            ]),
                            description: Some(err.to_string())
                        });

                        tracing::error!("Failed to run {:?} using wine: {err}", executable);
                    }
                }
            }

            #[allow(unused_must_use)]
            GeneralAppMsg::Toast { title, description } => {
                sender.output(Self::Output::Toast { title, description });
            }
        }
    }
}
