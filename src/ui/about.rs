use relm4::prelude::*;
use gtk::prelude::*;

use anime_launcher_sdk::VERSION as SDK_VERSION;
use anime_launcher_sdk::anime_game_core::VERSION as CORE_VERSION;

use crate::*;

lazy_static::lazy_static! {
    pub static ref APP_VERSION: String = if crate::APP_DEBUG && !crate::APP_VERSION.contains('-') {
        format!("{}-dev", crate::APP_VERSION)
    } else {
        crate::APP_VERSION.to_string()
    };
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
            set_version: &APP_VERSION,

            set_developers: &[
                "Nikita Podvirnyy https://github.com/krypt0nn"
            ],

            // Took patch credits from the CREDITS.md;
            // If you want to change your name, link, or have any other questions - contact me

            add_credit_section: (Some("Patch credits"), &[
                "@Krock https://notabug.org/Krock",
                "@y0soro https://notabug.org/y0soro",
                "@3Shain https://notabug.org/3Shain",
                "@timbuntu https://www.playonlinux.com/de/profil-95714.html",
                "@geearf2",
                "@SeppNel https://www.playonlinux.com/en/profil-95643.html",
                "@0x90 https://www.playonlinux.com/en/profil-96196.html",
                "@Th1nkCh3ck https://notabug.org/Th1nkCh3ck",
                "@humanik12",
                "@Makksim https://notabug.org/Makksim",
                "@Kowalski https://notabug.org/Kowalski",
                "@WerWolv",
                "@Various"
            ]),

            add_credit_section: (Some("An Anime Team"), &[
                "Nikita Podvirnyy https://github.com/krypt0nn",
                "Marie Piontek https://github.com/Mar0xy",
                "Luna Neff  https://github.com/lunaneff",
                "Renaud Lepage https://github.com/cybik",
                "Soham Nandy https://github.com/natimerry",
                "@mkrsym1 https://github.com/mkrsym1"
            ]),

            set_artists: &[
                "@nightany https://pinterest.com/pin/356206651788051017"
            ],

            set_translator_credits: &[
                "Русский, English — Nikita Podvirnyy https://github.com/krypt0nn",
                "Deutsch — Marie Piontek https://github.com/Mar0xy",
                "Français — @zeGolem https://github.com/zeGolem",
                "Español — Lautaro Garavano https://github.com/Rattlehead15",
                "Türkçe — @Kaozix https://github.com/Kaozix1776",
                "Türkçe — Kayra Nachfolger https://github.com/kayranachfolger",
                "Italiano — @QuazarOmega https://github.com/quazar-omega",
                "Indonesia — @yumekarisu https://github.com/yumekarisu",
                "简体中文 — Caibin Chen https://github.com/tigersoldier",
                "日本語 — @zozonteq https://github.com/zozonteq",
                // Hungarian?
                "Svenska — Jakob Fridesjö https://github.com/jakobfridesjo",
            ].join("\n"),

            set_debug_info: &[
                format!("Anime Launcher SDK: {SDK_VERSION}"),
                format!("Anime Game Core: {CORE_VERSION}"),
                String::new(),
                format!("gtk: {}.{}.{}", gtk::major_version(), gtk::minor_version(), gtk::micro_version()),
                format!("libadwaita: {}.{}.{}", adw::major_version(), adw::minor_version(), adw::micro_version()),
                format!("pango: {}", gtk::pango::version_string()),
                format!("cairo: {}", gtk::cairo::version_string())
            ].join("\n"),

            set_release_notes_version: &APP_VERSION,
            set_release_notes: &[
                "<p>Added</p>",

                "<ul>",
                    "<li>Added new gamescope version compatibility</li>",
                    "<li>Added \"launcher behavior\" option</li>",
                    "<li>Added \"kill game process\" button when chosen behavior keeps launcher window open</li>",
                    "<li>Bundled some icons into the app for consistency across the systems</li>",
                    "<li>Added better panics handler</li>",
                    "<li>Added Swedish</li>",
                "</ul>",

                "<p>Fixed</p>",

                "<ul>",
                    "<li>Fixed predownload button sensitivity</li>",
                "</ul>",

                "<p>Changed</p>",

                "<ul>",
                    "<li>Improved pre-downloads state checking</li>",
                    "<li>Replaced translation functions by `tr!` macro</li>",
                    "<li>Reworked app resources structure</li>",
                    "<li>Improved game running status check (wasn't working properly with Chinese client)</li>",
                "</ul>",
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
