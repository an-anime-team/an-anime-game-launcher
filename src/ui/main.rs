use relm4::{
    prelude::*,
    component::*,
    actions::*,
    MessageBroker
};

use gtk::prelude::*;
use adw::prelude::*;

use gtk::glib::clone;

use anime_launcher_sdk::config::launcher::LauncherStyle;
use anime_launcher_sdk::states::LauncherState;
use anime_launcher_sdk::wincompatlib::prelude::*;

use crate::*;
use crate::i18n::*;

use super::preferences::main::*;
use super::about::*;

relm4::new_action_group!(WindowActionGroup, "win");

relm4::new_stateless_action!(LauncherFolder, WindowActionGroup, "launcher_folder");
relm4::new_stateless_action!(GameFolder, WindowActionGroup, "game_folder");
relm4::new_stateless_action!(ConfigFile, WindowActionGroup, "config_file");
relm4::new_stateless_action!(DebugFile, WindowActionGroup, "debug_file");

relm4::new_stateless_action!(About, WindowActionGroup, "about");

static mut MAIN_WINDOW: Option<adw::Window> = None;
static mut PREFERENCES_WINDOW: Option<AsyncController<PreferencesApp>> = None;
static mut ABOUT_DIALOG: Option<Controller<AboutDialog>> = None;

pub struct App {
    toast_overlay: adw::ToastOverlay,

    loading: Option<Option<String>>,
    style: LauncherStyle,
    state: Option<LauncherState>,

    disable_buttons: bool
}

#[derive(Debug)]
pub enum AppMsg {
    UpdateLauncherState,

    /// Supposed to be called automatically on app's run when the latest game version
    /// was retrieved from the API
    SetGameDiff(Option<VersionDiff>),

    /// Supposed to be called automatically on app's run when the latest patch version
    /// was retrieved from remote repos
    SetPatch(Option<Patch>),

    /// Supposed to be called automatically on app's run when the launcher state was chosen
    SetLauncherState(Option<LauncherState>),

    SetLauncherStyle(LauncherStyle),
    SetLoadingStatus(Option<Option<String>>),

    OpenPreferences,
    ClosePreferences,
    DisableButtons(bool),
    PerformAction,

    Toast {
        title: String,
        description: Option<String>
    }
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    menu! {
        main_menu: {
            section! {
                "Launcher folder" => LauncherFolder,
                "Game folder" => GameFolder,
                "Config file" => ConfigFile,
                "Debug file" => DebugFile,
            },

            section! {
                "About" => About
            }
        }
    }

    view! {
        main_window = adw::Window {
            set_title: Some("An Anime Game Launcher"),

            #[watch]
            set_default_size: (
                match model.style {
                    LauncherStyle::Modern => 900,
                    LauncherStyle::Classic => 1094 // (w = 1280 / 730 * h, where 1280x730 is default background picture resolution)
                },
                match model.style {
                    LauncherStyle::Modern => 600,
                    LauncherStyle::Classic => 624
                }
            ),

            #[watch]
            set_css_classes: &{
                let mut classes = vec!["background", "csd"];

                if APP_DEBUG {
                    classes.push("devel");
                }

                match model.style {
                    LauncherStyle::Modern => (),
                    LauncherStyle::Classic => {
                        if model.loading.is_none() {
                            classes.push("classic-style");
                        }
                    }
                }

                classes
            },

            #[local_ref]
            toast_overlay -> adw::ToastOverlay {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        #[watch]
                        set_css_classes: match model.style {
                            LauncherStyle::Modern => &[""],
                            LauncherStyle::Classic => &["flat"]
                        },

                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            set_menu_model: Some(&main_menu)
                        }
                    },

                    adw::StatusPage {
                        set_title: "Loading data",
                        set_icon_name: Some(APP_ID),
                        set_vexpand: true,

                        #[watch]
                        set_description: match &model.loading {
                            Some(Some(desc)) => Some(desc),
                            Some(None) | None => None
                        },

                        #[watch]
                        set_visible: model.loading.is_some()
                    },

                    adw::PreferencesPage {
                        #[watch]
                        set_visible: model.loading.is_none(),

                        add = &adw::PreferencesGroup {
                            #[watch]
                            set_visible: model.style == LauncherStyle::Modern,

                            gtk::Image {
                                set_resource: Some("/org/app/images/icon.png"),
                                set_vexpand: true,
                                set_margin_top: 48
                            },

                            gtk::Label {
                                set_label: "An Anime Game Launcher",
                                set_margin_top: 32,
                                add_css_class: "title-1"
                            }
                        },

                        add = &adw::PreferencesGroup {
                            #[watch]
                            set_valign: match model.style {
                                LauncherStyle::Modern => gtk::Align::Center,
                                LauncherStyle::Classic => gtk::Align::End
                            },

                            #[watch]
                            set_width_request: match model.style {
                                LauncherStyle::Modern => -1,
                                LauncherStyle::Classic => 800
                            },

                            set_vexpand: true,

                            gtk::Box {
                                #[watch]
                                set_halign: match model.style {
                                    LauncherStyle::Modern => gtk::Align::Center,
                                    LauncherStyle::Classic => gtk::Align::End
                                },

                                #[watch]
                                set_height_request: match model.style {
                                    LauncherStyle::Modern => -1,
                                    LauncherStyle::Classic => 40
                                },

                                set_margin_top: 64,
                                set_spacing: 8,

                                // TODO: add tooltips

                                gtk::Button {
                                    #[watch]
                                    set_label: &match model.state {
                                        Some(LauncherState::Launch)                      => tr("launch"),
                                        Some(LauncherState::PredownloadAvailable { .. }) => tr("launch"),
                                        Some(LauncherState::PatchAvailable(_))           => tr("apply-patch"),
                                        Some(LauncherState::WineNotInstalled)            => tr("download-wine"),
                                        Some(LauncherState::PrefixNotExists)             => tr("create-prefix"),
                                        Some(LauncherState::VoiceUpdateAvailable(_))     => tr("update"),
                                        Some(LauncherState::VoiceOutdated(_))            => tr("update"),
                                        Some(LauncherState::VoiceNotInstalled(_))        => tr("download"),
                                        Some(LauncherState::GameUpdateAvailable(_))      => tr("update"),
                                        Some(LauncherState::GameOutdated(_))             => tr("update"),
                                        Some(LauncherState::GameNotInstalled(_))         => tr("download"),

                                        None => String::from("...")
                                    },

                                    #[watch]
                                    set_sensitive: match model.state.as_ref() {
                                        Some(LauncherState::GameOutdated { .. }) |
                                        Some(LauncherState::VoiceOutdated(_)) => false,

                                        Some(LauncherState::PatchAvailable(patch)) => match patch {
                                            Patch::NotAvailable |
                                            Patch::Outdated { .. } |
                                            Patch::Preparation { .. } => false,

                                            Patch::Testing { .. } |
                                            Patch::Available { .. } => true
                                        },

                                        Some(_) => true,

                                        None => false
                                    },

                                    #[watch]
                                    set_css_classes: match model.state.as_ref() {
                                        Some(LauncherState::GameOutdated { .. }) |
                                        Some(LauncherState::VoiceOutdated(_)) => &["warning"],

                                        Some(LauncherState::PatchAvailable(patch)) => match patch {
                                            Patch::NotAvailable |
                                            Patch::Outdated { .. } |
                                            Patch::Preparation { .. } => &["error"],

                                            Patch::Testing { .. } => &["warning"],
                                            Patch::Available { .. } => &["suggested-action"]
                                        },

                                        Some(_) => &["suggested-action"],

                                        None => &[]
                                    },

                                    #[watch]
                                    set_tooltip_text: Some(&match model.state.as_ref() {
                                        Some(LauncherState::GameOutdated { .. }) |
                                        Some(LauncherState::VoiceOutdated(_)) => tr("main-window--version-outdated-tooltip"),

                                        Some(LauncherState::PatchAvailable(patch)) => match patch {
                                            Patch::NotAvailable => tr("main-window--patch-unavailable-tooltip"),

                                            Patch::Outdated { .. } |
                                            Patch::Preparation { .. } => tr("main-window--patch-outdated-tooltip"),

                                            _ => String::new()
                                        },

                                        _ => String::new()
                                    }),

                                    #[watch]
                                    set_sensitive: !model.disable_buttons,

                                    set_hexpand: false,
                                    set_width_request: 200,

                                    connect_clicked => AppMsg::PerformAction
                                },

                                gtk::Button {
                                    #[watch]
                                    set_width_request: match model.style {
                                        LauncherStyle::Modern => -1,
                                        LauncherStyle::Classic => 40
                                    },

                                    #[watch]
                                    set_sensitive: !model.disable_buttons,

                                    set_icon_name: "emblem-system-symbolic",

                                    connect_clicked => AppMsg::OpenPreferences
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        _counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing main window");

        let model = App {
            toast_overlay: adw::ToastOverlay::new(),

            loading: Some(None),
            style: CONFIG.launcher.style,
            state: None,

            disable_buttons: false
        };

        let toast_overlay = &model.toast_overlay;

        let widgets = view_output!();

        let about_dialog_broker: MessageBroker<AboutDialog> = MessageBroker::new();

        unsafe {
            MAIN_WINDOW = Some(widgets.main_window.clone());

            PREFERENCES_WINDOW = Some(PreferencesApp::builder()
                .launch(widgets.main_window.clone().into())
                .forward(sender.input_sender(), std::convert::identity));

            ABOUT_DIALOG = Some(AboutDialog::builder()
                .transient_for(widgets.main_window.clone())
                .launch_with_broker((), &about_dialog_broker)
                .detach());
        }

        let group = RelmActionGroup::<WindowActionGroup>::new();

        // TODO: reduce code somehow

        group.add_action::<LauncherFolder>(&RelmAction::new_stateless(clone!(@strong sender => move |_| {
            if let Err(err) = std::process::Command::new("xdg-open").arg(LAUNCHER_FOLDER.as_path()).spawn() {
                sender.input(AppMsg::Toast {
                    title: tr("launcher-folder-opening-error"),
                    description: Some(err.to_string())
                });

                tracing::error!("Failed to open launcher folder: {err}");
            }
        })));

        group.add_action::<GameFolder>(&RelmAction::new_stateless(clone!(@strong sender => move |_| {
            if let Err(err) = std::process::Command::new("xdg-open").arg(&CONFIG.game.path).spawn() {
                sender.input(AppMsg::Toast {
                    title: tr("game-folder-opening-error"),
                    description: Some(err.to_string())
                });

                tracing::error!("Failed to open game folder: {err}");
            }
        })));

        group.add_action::<ConfigFile>(&RelmAction::new_stateless(clone!(@strong sender => move |_| {
            if let Some(file) = anime_launcher_sdk::consts::config_file() {
                if let Err(err) = std::process::Command::new("xdg-open").arg(file).spawn() {
                    sender.input(AppMsg::Toast {
                        title: tr("config-file-opening-error"),
                        description: Some(err.to_string())
                    });

                    tracing::error!("Failed to open config file: {err}");
                }
            }
        })));

        group.add_action::<DebugFile>(&RelmAction::new_stateless(clone!(@strong sender => move |_| {
            if let Err(err) = std::process::Command::new("xdg-open").arg(DEBUG_FILE.as_os_str()).spawn() {
                sender.input(AppMsg::Toast {
                    title: tr("debug-file-opening-error"),
                    description: Some(err.to_string())
                });

                tracing::error!("Failed to open debug file: {err}");
            }
        })));

        group.add_action::<About>(&RelmAction::new_stateless(move |_| {
            about_dialog_broker.send(AboutDialogMsg::Show);
        }));

        widgets.main_window.insert_action_group("win", Some(&group.into_action_group()));

        tracing::info!("Main window initialized");

        let download_picture = model.style == LauncherStyle::Classic && !KEEP_BACKGROUND_FILE.exists();

        // Initialize some heavy tasks
        std::thread::spawn(move || {
            tracing::info!("Initializing heavy tasks");

            // Download background picture if needed

            if download_picture {
                sender.input(AppMsg::SetLoadingStatus(Some(Some(tr("downloading-background-picture")))));

                if let Err(err) = crate::background::download_background() {
                    tracing::error!("Failed to download background picture");

                    sender.input(AppMsg::Toast {
                        title: tr("background-downloading-failed"),
                        description: Some(err.to_string())
                    });
                }
            }

            // Update initial game version status

            sender.input(AppMsg::SetLoadingStatus(Some(Some(tr("loading-game-version")))));

            sender.input(AppMsg::SetGameDiff(match GAME.try_get_diff() {
                Ok(diff) => Some(diff),
                Err(err) => {
                    tracing::error!("Failed to find game diff: {err}");

                    sender.input(AppMsg::Toast {
                        title: tr("game-diff-finding-error"),
                        description: Some(err.to_string())
                    });

                    None
                }
            }));

            tracing::info!("Updated game version status");

            // Update initial patch status

            sender.input(AppMsg::SetLoadingStatus(Some(Some(tr("loading-patch-status")))));

            sender.input(AppMsg::SetPatch(match Patch::try_fetch(&CONFIG.patch.servers, None) {
                Ok(patch) => Some(patch),
                Err(err) => {
                    tracing::error!("Failed to fetch patch info: {err}");

                    sender.input(AppMsg::Toast {
                        title: tr("patch-info-fetching-error"),
                        description: Some(err.to_string())
                    });

                    None
                }
            }));

            tracing::info!("Updated patch status");

            // Update launcher state
            sender.input(AppMsg::UpdateLauncherState);

            // Mark app as loaded
            unsafe {
                crate::READY = true;
            }

            tracing::info!("App is ready");
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        tracing::debug!("Called main window event: {:?}", msg);

        match msg {
            AppMsg::UpdateLauncherState => {
                sender.input(AppMsg::SetLoadingStatus(Some(Some(tr("loading-launcher-state")))));

                let updater = clone!(@strong sender => move |state| {
                    use anime_launcher_sdk::states::StateUpdating;
    
                    match state {
                        StateUpdating::Game => {
                            sender.input(AppMsg::SetLoadingStatus(Some(Some(tr("loading-launcher-state--game")))));
                        }
    
                        StateUpdating::Voice(locale) => {
                            sender.input(AppMsg::SetLoadingStatus(Some(Some(tr_args("loading-launcher-state--voice", [
                                ("locale", locale.to_name().to_owned().into())
                            ])))));
                        }
    
                        StateUpdating::Patch => {
                            sender.input(AppMsg::SetLoadingStatus(Some(Some(tr("loading-launcher-state--patch")))));
                        }
                    }
                });

                let state = match LauncherState::get_from_config(updater) {
                    Ok(state) => Some(state),
                    Err(err) => {
                        tracing::error!("Failed to update launcher state: {err}");

                        sender.input(AppMsg::Toast {
                            title: tr("launcher-state-updating-error"),
                            description: Some(err.to_string())
                        });
    
                        None
                    }
                };

                sender.input(AppMsg::SetLauncherState(state));
                sender.input(AppMsg::SetLoadingStatus(None));
            }

            #[allow(unused_must_use)]
            AppMsg::SetGameDiff(diff) => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().sender().send(PreferencesAppMsg::SetGameDiff(diff));
            }

            #[allow(unused_must_use)]
            AppMsg::SetPatch(patch) => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().sender().send(PreferencesAppMsg::SetPatch(patch));
            }

            AppMsg::SetLauncherState(state) => {
                self.state = state;
            }

            AppMsg::SetLoadingStatus(status) => {
                self.loading = status;
            }

            AppMsg::SetLauncherStyle(style) => {
                self.style = style;
            }

            AppMsg::OpenPreferences => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().widget().show();
            }

            AppMsg::ClosePreferences => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().widget().hide();
            }

            AppMsg::DisableButtons(state) => {
                self.disable_buttons = state;
            }

            AppMsg::PerformAction => unsafe {
                match self.state.as_ref().unwrap_unchecked() {
                    LauncherState::PatchAvailable(Patch::NotAvailable) |
                    LauncherState::PredownloadAvailable { .. } |
                    LauncherState::Launch  => {
                        if let Err(err) = anime_launcher_sdk::game::run() {
                            tracing::error!("Failed to launch game: {err}");

                            sender.input(AppMsg::Toast {
                                title: tr("game-launching-failed"),
                                description: Some(err.to_string())
                            });
                        }

                        else {
                            MAIN_WINDOW.as_ref().unwrap_unchecked().hide();

                            std::thread::sleep(std::time::Duration::from_secs(2));

                            /*if config.launcher.discord_rpc.enabled {
                                this.widgets.preferences_stack.enhancements_page.discord_rpc.update(RpcUpdates::Connect);
                            }*/

                            loop {
                                std::thread::sleep(std::time::Duration::from_secs(3));

                                match std::process::Command::new("ps").arg("-A").stdout(std::process::Stdio::piped()).output() {
                                    Ok(output) => {
                                        let output = String::from_utf8_lossy(&output.stdout);

                                        if !output.contains("GenshinImpact.e") && !output.contains("unlocker.exe") {
                                            break;
                                        }
                                    }

                                    Err(_) => break
                                }
                            }

                            /*if config.launcher.discord_rpc.enabled {
                                this.widgets.preferences_stack.enhancements_page.discord_rpc.update(RpcUpdates::Disconnect);
                            }*/

                            MAIN_WINDOW.as_ref().unwrap_unchecked().show();
                        }
                    }

                    LauncherState::PatchAvailable(patch) => {
                        match patch.to_owned() {
                            Patch::NotAvailable |
                            Patch::Outdated { .. } |
                            Patch::Preparation { .. } => unreachable!(),

                            Patch::Testing { version, host, .. } |
                            Patch::Available { version, host, .. } => {
                                self.disable_buttons = true;

                                let config = config::get().unwrap();

                                std::thread::spawn(move || {
                                    let applier = PatchApplier::new(&config.patch.path);

                                    let mut synced = false;

                                    match applier.is_sync_with(&host) {
                                        Ok(true) => synced = true,

                                        Ok(false) => {
                                            match applier.sync(host) {
                                                Ok(true) => synced = true,

                                                Ok(false) => {
                                                    sender.input(AppMsg::Toast {
                                                        title: tr("patch-sync-failed"),
                                                        description: None
                                                    });
                                                }

                                                Err(err) => {
                                                    sender.input(AppMsg::Toast {
                                                        title: tr("patch-sync-failed"),
                                                        description: Some(err.to_string())
                                                    });
                                                }
                                            }
                                        }

                                        Err(err) => {
                                            sender.input(AppMsg::Toast {
                                                title: tr("patch-state-check-failed"),
                                                description: Some(err.to_string())
                                            });
                                        }
                                    }

                                    if synced {
                                        if let Err(err) = applier.apply(&config.game.path, version, config.patch.root) {
                                            sender.input(AppMsg::Toast {
                                                title: tr("game-patching-error"),
                                                description: Some(err.to_string())
                                            });
                                        }
                                    }

                                    sender.input(AppMsg::DisableButtons(false));
                                    sender.input(AppMsg::UpdateLauncherState);
                                });
                            }
                        }
                    }

                    LauncherState::WineNotInstalled => todo!(),

                    LauncherState::PrefixNotExists => {
                        let config = config::get().unwrap();

                        match config.try_get_wine_executable() {
                            Some(wine) => {
                                sender.input(AppMsg::DisableButtons(true));

                                std::thread::spawn(move || {
                                    let wine = Wine::from_binary(wine)
                                        .with_loader(WineLoader::Current)
                                        .with_arch(WineArch::Win64);

                                    if let Err(err) = wine.update_prefix(&config.game.wine.prefix) {
                                        sender.input(AppMsg::Toast {
                                            title: tr("wine-prefix-update-failed"),
                                            description: Some(err.to_string())
                                        });
                                    }

                                    sender.input(AppMsg::DisableButtons(true));
                                    sender.input(AppMsg::UpdateLauncherState);
                                });
                            }

                            None => sender.input(AppMsg::Toast {
                                title: tr("failed-get-selected-wine"),
                                description: None
                            })
                        }
                    }

                    LauncherState::VoiceUpdateAvailable(_) => todo!(),
                    LauncherState::VoiceNotInstalled(_) => todo!(),
                    LauncherState::GameUpdateAvailable(_) => todo!(),
                    LauncherState::GameNotInstalled(_) => todo!(),

                    LauncherState::VoiceOutdated(_) |
                    LauncherState::GameOutdated(_) => ()
                }
            }

            AppMsg::Toast { title, description } => unsafe {
                let toast = adw::Toast::new(&title);

                toast.set_timeout(5);

                if let Some(description) = description {
                    toast.set_button_label(Some(&tr("details")));

                    let dialog = adw::MessageDialog::new(Some(MAIN_WINDOW.as_ref().unwrap_unchecked()), Some(&title), Some(&description));

                    dialog.add_response("close", &tr("close"));
                    dialog.add_response("save", &tr("save"));

                    dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);

                    #[allow(unused_must_use)]
                    dialog.connect_response(Some("save"), |_, _| {
                        let result = std::process::Command::new("xdg-open")
                            .arg(crate::DEBUG_FILE.as_os_str())
                            .output();

                        if let Err(err) = result {
                            tracing::error!("Failed to open debug file: {}", err);
                        }
                    });

                    toast.connect_button_clicked(move |_| {
                        dialog.show();
                    });
                }

                self.toast_overlay.add_toast(&toast);
            }
        }
    }
}
