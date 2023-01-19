use relm4::prelude::*;

use gtk::prelude::*;

use anime_launcher_sdk::anime_game_core::{VERSION as CORE_VERSION, curl_sys};

lazy_static::lazy_static! {
    static ref CURL_INFO: curl_sys::Version = curl_sys::Version::get();
}

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

            add_credit_section: (Some("Logo"), &[
                "@nightany https://pinterest.com/pin/356206651788051017"
            ]),

            add_credit_section: (Some("An Anime Team"), &[
                "@Marie https://github.com/Mar0xy",
                "@lane https://github.com/laurinneff"
            ]),

            set_debug_info: &[
                format!("Anime Game core library version: {CORE_VERSION}"),
                format!("Curl version: {}", CURL_INFO.version()),
                format!("SSL version: {}", CURL_INFO.ssl_version().unwrap_or("?")),
                String::new(),
                format!("GTK version: {}.{}.{}", gtk::major_version(), gtk::minor_version(), gtk::micro_version()),
                format!("Libadwaita version: {}.{}.{}", adw::major_version(), adw::minor_version(), adw::micro_version()),
                format!("Pango version: {}", gtk::pango::version_string()),
                format!("Cairo version: {}", gtk::cairo::version_string()),
            ].join("\n"),

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
