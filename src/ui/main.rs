use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::i18n::tr;

use super::preferences::main::App as PreferencesApp;

/// Sets to `true` when the `App` component is ready (fully initialized)
pub static mut READY: bool = false;

// TODO: get rid of using this function in all the components' events
//       e.g. by converting preferences pages into Relm4 Components
pub fn is_ready() -> bool {
    unsafe { READY }
}

pub struct App {
    preferences_window: Controller<PreferencesApp>
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

    view! {
        main_window = adw::Window {
            set_title: Some("An Anime Game Launcher"),
            set_default_size: (900, 600),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {},

                adw::PreferencesPage {
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
        let widgets = view_output!();

        let model = App {
            preferences_window: PreferencesApp::builder()
                .launch(widgets.main_window.clone().into())
                .detach()
        };

        unsafe {
            READY = true;
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenPreferences => {
                self.preferences_window.widgets().preferences_window.show();
            }

            AppMsg::ClosePreferences => {
                self.preferences_window.widgets().preferences_window.hide();
            }
        }
    }
}
