use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use crate::i18n::*;
use super::main::*;

pub struct FinishApp;

#[derive(Debug, Clone)]
pub enum FinishAppMsg {
    Restart,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for FinishApp {
    type Init = ();
    type Input = FinishAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: &tr("finish-title"),
                    add_css_class: "title-1"
                },

                gtk::Label {
                    set_label: &tr("finish-message"),
    
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
                        set_label: &tr("restart"),
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => FinishAppMsg::Restart
                    },

                    gtk::Button {
                        set_label: &tr("exit"),
                        add_css_class: "pill",

                        connect_clicked => FinishAppMsg::Exit
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

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            FinishAppMsg::Restart => {
                std::process::Command::new(std::env::current_exe().unwrap()).spawn().unwrap();

                // TODO: relm4 has some function for it
                std::process::exit(0);
            }

            FinishAppMsg::Exit => {
                // TODO: relm4 has some function for it
                std::process::exit(0);
            }
        }
    }
}
