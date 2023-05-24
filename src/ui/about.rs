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

            add_credit_section: (Some("Patch developer"), &[
                "@Krock https://notabug.org/Krock/dawn"
            ]),

            add_credit_section: (Some("An Anime Team"), &[
                "Nikita Podvirnyy https://github.com/krypt0nn",
                "@Marie https://github.com/Mar0xy",
                "@lane https://github.com/laurinneff",
                "@jiro-too https://github.com/jiro-too",
                "@cybik https://github.com/cybik",
                "@mkrsym1 https://github.com/mkrsym1"
            ]),

            set_artists: &[
                "@nightany https://pinterest.com/pin/356206651788051017"
            ],

            set_translator_credits: &[
                "Русский, English — Nikita Podvirnyy https://github.com/krypt0nn",
                "Deutsch — @Marie https://github.com/Mar0xy",
                "Español — Lautaro Garavano https://github.com/Rattlehead15",
                "Français — @zeGolem https://github.com/zeGolem",
                "Türkçe — @Kaozix https://github.com/Kaozix1776",
                "Türkçe — Kayra Nachfolger https://github.com/kayranachfolger",
                "Italiano - @QuazarOmega https://github.com/quazar-omega",
                "Indonesia - @yumekarisu https://github.com/yumekarisu",
                "简体中文 — Caibin Chen https://github.com/tigersoldier"
            ].join("\n"),

            set_debug_info: &[
                format!("Anime Launcher SDK: {SDK_VERSION}"),
                format!("Anime Game Core: {CORE_VERSION}"),
                String::new(),
                format!("GTK: {}.{}.{}", gtk::major_version(), gtk::minor_version(), gtk::micro_version()),
                format!("libadwaita: {}.{}.{}", adw::major_version(), adw::minor_version(), adw::micro_version()),
                format!("pango: {}", gtk::pango::version_string()),
                format!("cairo: {}", gtk::cairo::version_string()),
            ].join("\n"),

            set_release_notes_version: &APP_VERSION,
            set_release_notes: &[
                "<p>Added</p>",

                "<ul>",
                    "<li>Added Italian</li>",
                    "<li>Added Indonesian</li>",
                    "<li>Added dynamic main button icon switching</li>",
                    "<li>Set button label as \"Resume\" when the diff is part downloaded</li>",
                    "<li>Added options to use wine / gstreamer shared libraries from selected wine build. These options will configure `LD_LIBRARY_PATH` and `GST_PLUGIN_PATH` environment variables</li>",
                    "<li>Added setting of `LC_ALL` in wine lang setting</li>",
                    "<li>Added `LAUNCHER_REQUESTS_TIMEOUT` environment variable</li>",
                    "<li>Added option to disable main patch applying</li>",
                "</ul>",

                "<p>Fixed</p>",

                "<ul>",
                    "<li>Fixed session applying on each launcher start</li>",
                    "<li>Fixed predownload button ui</li>",
                    "<li>Fixed proton builds integration with sandbox</li>",
                    "<li>Fixed compatibility between sessions manager and sandbox</li>",
                    "<li>Fixed sandboxing of inexisting folders</li>",
                "</ul>",

                "<p>Changed</p>",

                "<ul>",
                    "<li>Apply selected session before launching the game. This will properly save your game session when you switch between wine prefixes</li>",
                    "<li>Redesigned main button</li>",
                    "<li>Used `whatadistro` to identify recommended package manager in the first run window</li>",
                    "<li>Moved a lot of settings to separate page</li>",
                    "<li>Set fsr quality mode in enhancements settings instead of strength</li>",
                    "<li>Updated fps unlocker data</li>",
                    "<li>Made temporary workaround to the game API changes</li>",
                    "<li>Increased default requests timeout to 8 seconds</li>",
                    "<li>Updated minreq to support `http_proxy`-like variables</li>",
                    "<li>Disabled xlua patch applying by default</li>",
                "</ul>",

                "<p>Removed</p>",

                "<ul>",
                    "<li>Removed Futex2 wine sync option</li>",
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
