use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::i18n::tr;

pub struct AboutDialog {
    visible: bool
}

#[derive(Debug)]
pub enum AppMsg {
    Show,
    Hide
}

#[relm4::component(pub)]
impl SimpleComponent for AboutDialog {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        dialog = adw::AboutWindow {
            set_application_name: "An Anime Game Launcher",
            set_application_icon: "moe.launcher.an-anime-game-launcher-gtk",

            set_website: "https://github.com/an-anime-team/an-anime-game-launcher-gtk",
            set_issue_url: "https://github.com/an-anime-team/an-anime-game-launcher-gtk/issues",

            set_modal: true,
            set_hide_on_close: true,

            #[watch]
            set_visible: model.visible,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Hide);

                gtk::Inhibit(false)
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AboutDialog {
            visible: false
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Show => {
                self.visible = true;
            }

            AppMsg::Hide => {
                self.visible = false;
            }
        }
    }
}
