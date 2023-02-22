use relm4::{
    prelude::*,
    actions::*,
    MessageBroker
};

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::config::launcher::LauncherStyle;

use crate::*;
use crate::i18n::tr;

use super::preferences::main::*;
use super::about::*;

relm4::new_action_group!(WindowActionGroup, "win");

relm4::new_stateless_action!(LauncherFolder, WindowActionGroup, "launcher_folder");
relm4::new_stateless_action!(GameFolder, WindowActionGroup, "game_folder");
relm4::new_stateless_action!(ConfigFile, WindowActionGroup, "config_file");

relm4::new_stateless_action!(About, WindowActionGroup, "about");

static mut PREFERENCES_WINDOW: Option<Controller<PreferencesApp>> = None;
static mut ABOUT_DIALOG: Option<Controller<AboutDialog>> = None;

pub struct App {
    loading: Option<Option<String>>,
    style: LauncherStyle
}

#[derive(Debug)]
pub enum AppMsg {
    /// Supposed to be called automatically on app's run when the latest game version
    /// was retrieved from the API
    UpdateGameDiff(Option<VersionDiff>),

    /// Supposed to be called automatically on app's run when the latest patch version
    /// was retrieved from remote repos
    UpdatePatch(Option<Patch>),

    PerformAction,
    OpenPreferences,
    ClosePreferences,
    UpdateLauncherStyle(LauncherStyle)
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
            add_css_class: match model.style {
                LauncherStyle::Modern => "",
                LauncherStyle::Classic => {
                    if model.loading.is_none() {
                        "classic-style"
                    } else {
                        ""
                    }
                }
            },

            #[watch]
            remove_css_class: match model.style {
                LauncherStyle::Modern => "classic-style",
                LauncherStyle::Classic => ""
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    #[watch]
                    add_css_class: match model.style {
                        LauncherStyle::Modern => "",
                        LauncherStyle::Classic => "flat"
                    },

                    #[watch]
                    remove_css_class: match model.style {
                        LauncherStyle::Modern => "flat",
                        LauncherStyle::Classic => ""
                    },

                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&main_menu)
                    }
                },

                adw::StatusPage {
                    set_title: "Loading data",
                    set_icon_name: Some("process-working"),
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

                            gtk::Button {
                                set_label: &tr("launch"),
                                set_hexpand: false,
                                set_width_request: 200,
                                add_css_class: "suggested-action",

                                connect_clicked => AppMsg::PerformAction
                            },

                            gtk::Button {
                                #[watch]
                                set_width_request: match model.style {
                                    LauncherStyle::Modern => -1,
                                    LauncherStyle::Classic => 40
                                },

                                set_icon_name: "emblem-system-symbolic",

                                connect_clicked => AppMsg::OpenPreferences
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
            loading: None,
            style: CONFIG.launcher.style
        };

        let widgets = view_output!();

        if crate::APP_DEBUG {
            widgets.main_window.add_css_class("devel");
        }

        let about_dialog_broker: MessageBroker<AboutDialog> = MessageBroker::new();

        unsafe {
            PREFERENCES_WINDOW = Some(PreferencesApp::builder()
                .launch(widgets.main_window.clone().into())
                .forward(sender.input_sender(), std::convert::identity));

            ABOUT_DIALOG = Some(AboutDialog::builder()
                .transient_for(widgets.main_window.clone())
                .launch_with_broker((), &about_dialog_broker)
                .detach());
        }

        let group = RelmActionGroup::<WindowActionGroup>::new();

        // TODO
        group.add_action::<LauncherFolder>(&RelmAction::new_stateless(move |_| {
            println!("Open launcher folder!");
        }));

        group.add_action::<GameFolder>(&RelmAction::new_stateless(move |_| {
            println!("Open game folder!");
        }));

        group.add_action::<ConfigFile>(&RelmAction::new_stateless(move |_| {
            println!("Open config file!");
        }));

        group.add_action::<About>(&RelmAction::new_stateless(move |_| {
            about_dialog_broker.send(AboutDialogMsg::Show);
        }));

        widgets.main_window.insert_action_group("win", Some(&group.into_action_group()));

        tracing::info!("Main window initialized");

        // Initialize some heavy tasks
        std::thread::spawn(move || {
            tracing::info!("Initializing heavy tasks");

            // Update initial game version status
            sender.input(AppMsg::UpdateGameDiff(match GAME.try_get_diff() {
                Ok(diff) => Some(diff),
                Err(err) => {
                    tracing::error!("Failed to get game diff {err}");
            
                    None
                }
            }));

            tracing::info!("Updated game version status");

            // Update initial patch status
            sender.input(AppMsg::UpdatePatch(match Patch::try_fetch(&CONFIG.patch.servers, None) {
                Ok(patch) => Some(patch),
                Err(err) => {
                    tracing::error!("Failed to fetch patch info {err}");
            
                    None
                }
            }));

            tracing::info!("Updated patch status");

            unsafe {
                crate::READY = true;
            }

            tracing::info!("App is ready");
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called main window event: {:?}", msg);

        match msg {
            #[allow(unused_must_use)]
            AppMsg::UpdateGameDiff(diff) => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().sender().send(PreferencesAppMsg::UpdateGameDiff(diff));
            },

            #[allow(unused_must_use)]
            AppMsg::UpdatePatch(patch) => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().sender().send(PreferencesAppMsg::UpdatePatch(patch));
            },

            AppMsg::PerformAction => {
                anime_launcher_sdk::game::run().expect("Failed to run the game");
            }

            AppMsg::OpenPreferences => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().widget().show();
            }

            AppMsg::ClosePreferences => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().widget().hide();
            }

            AppMsg::UpdateLauncherStyle(style) => {
                self.style = style;
            }
        }
    }
}
