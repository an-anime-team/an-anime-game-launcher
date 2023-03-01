use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::is_available;

use crate::i18n::*;
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
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: &tr("tos-violation-warning"),
                    add_css_class: "title-1"
                }
            },

            add = &adw::PreferencesGroup {
                gtk::Label {
                    set_label: &tr("tos-violation-warning-message"),
                    set_wrap: true,
                    set_selectable: true
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
                        set_label: &tr("continue"),
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => TosWarningAppMsg::Continue
                    },

                    gtk::Button {
                        set_label: &tr("exit"),
                        add_css_class: "pill",

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

            TosWarningAppMsg::Exit => relm4::main_application().quit()
        }
    }
}
