use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use crate::*;

use super::main::FirstRunAppMsg;

pub struct WelcomeApp;

#[derive(Debug, Clone)]
pub enum WelcomeAppMsg {
    Continue
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for WelcomeApp {
    type Init = ();
    type Input = WelcomeAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Picture {
                    set_resource: Some(&format!("{APP_RESOURCE_PATH}/icons/hicolor/scalable/apps/{APP_ID}.png")),
                    set_height_request: 128
                },

                gtk::Label {
                    set_label: "An Anime Game Launcher",
                    set_margin_top: 32,
                    add_css_class: "title-1"
                },

                gtk::Label {
                    set_label: &tr!("welcome-page-message"),
    
                    set_justify: gtk::Justification::Center,
                    set_wrap: true,
                    set_margin_top: 32
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
                        set_label: &tr!("continue"),
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => WelcomeAppMsg::Continue
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
            WelcomeAppMsg::Continue => {
                sender.output(Self::Output::ScrollToTosWarning);
            }
        }
    }
}
