use relm4::prelude::*;

use gtk::prelude::*;

use anime_launcher_sdk::VERSION as SDK_VERSION;

use anime_launcher_sdk::anime_game_core::{
    VERSION as CORE_VERSION,
    curl_sys
};

use crate::*;

lazy_static::lazy_static! {
    static ref CURL_INFO: curl_sys::Version = curl_sys::Version::get();
}

#[derive(Debug)]
pub struct AboutDialog {
    visible: bool
}

#[derive(Debug)]
pub enum AboutDialogMsg {
    Show,
    Hide
}

#[relm4::component(pub)]
impl SimpleComponent for AboutDialog {
    type Init = ();
    type Input = AboutDialogMsg;
    type Output = ();

    view! {
        dialog = adw::AboutWindow {
            set_application_name: "An Anime Game Launcher",
            set_application_icon: APP_ID,

            set_website: "https://github.com/an-anime-team/an-anime-game-launcher",
            set_issue_url: "https://github.com/an-anime-team/an-anime-game-launcher/issues",

            set_license_type: gtk::License::Gpl30,

            set_version: &{
                // Debug build & build's version doesn't contain any suffix (-dev, -beta, etc)
                if crate::APP_DEBUG && !crate::APP_VERSION.contains('-') {
                    format!("{}-dev", crate::APP_VERSION)
                }
                
                else {
                    crate::APP_VERSION.to_string()
                }
            },

            set_developers: &[
                "Nikita Podvirnyy https://github.com/krypt0nn"
            ],

            add_credit_section: (Some("An Anime Team"), &[
                "@Marie https://github.com/Mar0xy",
                "@lane https://github.com/laurinneff",
                "@jiro-too https://github.com/jiro-too",
                "@cybik https://github.com/cybik"
            ]),

            set_artists: &[
                "@nightany https://pinterest.com/pin/356206651788051017"
            ],

            set_translator_credits: &[
                "Русский, English — Nikita Podvirnyy https://github.com/krypt0nn",
                "Deutsch — @Marie https://github.com/Mar0xy",
                "Español — Lautaro Garavano https://github.com/Rattlehead15",
                "Français — @zeGolem https://github.com/zeGolem",
                "Türk — @Kaozix https://github.com/Kaozix1776",
                "简体中文 — Caibin Chen https://github.com/tigersoldier"
            ].join("\n"),

            set_debug_info: &[
                format!("Anime Launcher SDK: {SDK_VERSION}"),
                format!("Anime Game Core: {CORE_VERSION}"),
                String::new(),
                format!("curl: {}", CURL_INFO.version()),
                format!("SSL: {}", CURL_INFO.ssl_version().unwrap_or("?")),
                String::new(),
                format!("GTK: {}.{}.{}", gtk::major_version(), gtk::minor_version(), gtk::micro_version()),
                format!("libadwaita: {}.{}.{}", adw::major_version(), adw::minor_version(), adw::micro_version()),
                format!("pango: {}", gtk::pango::version_string()),
                format!("cairo: {}", gtk::cairo::version_string()),
            ].join("\n"),

            set_modal: true,
            set_hide_on_close: true,

            #[watch]
            set_visible: model.visible,

            connect_close_request[sender] => move |_| {
                sender.input(AboutDialogMsg::Hide);

                gtk::Inhibit(false)
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing about dialog");

        let model = Self {
            visible: false
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AboutDialogMsg::Show => {
                self.visible = true;
            }

            AboutDialogMsg::Hide => {
                self.visible = false;
            }
        }
    }
}
