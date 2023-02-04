use relm4::{
    prelude::*,
    component::*,
    actions::*,
    MessageBroker
};

use gtk::prelude::*;
use adw::prelude::*;

use crate::i18n::tr;

use super::preferences::main::App as PreferencesApp;
use super::about::{AboutDialog, AppMsg as AboutDialogMsg};

relm4::new_action_group!(WindowActionGroup, "win");

relm4::new_stateless_action!(LauncherFolder, WindowActionGroup, "launcher_folder");
relm4::new_stateless_action!(GameFolder, WindowActionGroup, "game_folder");
relm4::new_stateless_action!(ConfigFile, WindowActionGroup, "config_file");

relm4::new_stateless_action!(About, WindowActionGroup, "about");

static mut PREFERENCES_WINDOW: Option<AsyncController<PreferencesApp>> = None;
static mut ABOUT_DIALOG: Option<Controller<AboutDialog>> = None;

pub struct App {
    loading: Option<Option<String>>
}

#[derive(Debug)]
pub enum AppMsg {
    OpenPreferences,
    ClosePreferences
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
            set_default_size: (900, 600),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
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
                        set_valign: gtk::Align::Center,
                        set_vexpand: true,

                        gtk::Box {
                            set_halign: gtk::Align::Center,
                            set_margin_top: 64,
                            set_spacing: 8,

                            gtk::Button {
                                set_label: &tr("launch"),
                                set_hexpand: false,
                                set_width_request: 200,
                                add_css_class: "suggested-action",

                                connect_clicked => |_| {
                                    anime_launcher_sdk::game::run().expect("Failed to run the game");
                                }
                            },

                            gtk::Button {
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
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing main window");

        let model = App {
            loading: None
        };

        let widgets = view_output!();

        if *crate::APP_DEBUG {
            widgets.main_window.add_css_class("devel");
        }

        let about_dialog_broker: MessageBroker<AboutDialog> = MessageBroker::new();

        unsafe {
            PREFERENCES_WINDOW = Some(PreferencesApp::builder()
                .launch(widgets.main_window.clone().into())
                .detach());

            ABOUT_DIALOG = Some(AboutDialog::builder()
                .transient_for(widgets.main_window.clone())
                .launch_with_broker((), &about_dialog_broker)
                .detach());
        }

        let group = RelmActionGroup::<WindowActionGroup>::new();

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

        unsafe {
            crate::READY = true;
        }

        tracing::info!("Main window initialized. App is ready");

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called main window event: {:?}", msg);

        match msg {
            AppMsg::OpenPreferences => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().widget().show();
            }

            AppMsg::ClosePreferences => unsafe {
                PREFERENCES_WINDOW.as_ref().unwrap_unchecked().widget().hide();
            }
        }
    }
}
