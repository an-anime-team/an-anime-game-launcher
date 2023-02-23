use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::is_available;

use super::main::FirstRunAppMsg;

pub struct TosWarningApp;

#[derive(Debug, Clone)]
pub enum TosWarningAppMsg {
    Continue,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for TosWarningApp {
    type Init = ();
    type Input = TosWarningAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                gtk::Label {
                    set_label: "ToS violation warning",
                    set_margin_top: 8,
                    add_css_class: "title-1"
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_top: 32,
                    set_spacing: 12,
                    set_hexpand: true,

                    // TODO: use some kind of multiline text field

                    gtk::Label {
                        set_label: "This launcher is an unofficial tool, in no way related to miHoYo nor COGNOSPHERE.",

                        set_wrap: true,
                        set_halign: gtk::Align::Center
                    },

                    gtk::Label {
                        set_label: "This tool is designed to facilitate playing Genshin Impact on Linux, and was built with the sole purpose of installing and running the game with less hassle.",

                        set_wrap: true,
                        set_halign: gtk::Align::Center
                    },

                    gtk::Label {
                        set_label: "It does so by using existing components and making the experience simple for the user.",

                        set_wrap: true,
                        set_halign: gtk::Align::Center
                    },

                    gtk::Label {
                        set_label: "However, some components used here likely break the miHoYo Terms of Service for Genshin Impact.",

                        set_wrap: true,
                        set_halign: gtk::Align::Center
                    },

                    gtk::Label {
                        set_label: "If you are using this launcher, your player account could become identified as TOS-non-compliant by miHoYo/COGNOSPHERE.",

                        set_wrap: true,
                        set_halign: gtk::Align::Center
                    },

                    gtk::Label {
                        set_label: "If this happens, as your account would be disobeying TOS, miHoYo/COGNOSPHERE are free to do what they want. Including banning.",

                        set_wrap: true,
                        set_halign: gtk::Align::Center
                    },

                    gtk::Label {
                        set_label: "If you understand the risk of trying to play the game in an unofficial capacity, press OK and let's go researching the world of Teyvat!",

                        set_wrap: true,
                        set_halign: gtk::Align::Center
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,
    
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 8,
    
                    gtk::Button {
                        set_label: "Continue",
                        add_css_class: "suggested-action",

                        connect_clicked => TosWarningAppMsg::Continue
                    },

                    gtk::Button {
                        set_label: "Exit",

                        connect_clicked => TosWarningAppMsg::Exit
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
        let model = Self;
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            #[allow(unused_must_use)]
            TosWarningAppMsg::Continue => {
                if is_available("git") && is_available("xdelta3") {
                    sender.output(Self::Output::ScrollToDefaultPaths);
                } else {
                    sender.output(Self::Output::ScrollToDependencies);
                }
            }

            TosWarningAppMsg::Exit => {
                // TODO: relm4 has some function for it
                std::process::exit(0);
            }
        }
    }
}
