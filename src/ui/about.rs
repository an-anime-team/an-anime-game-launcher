// SPDX-License-Identifier: GPL-3.0-or-later
//
// an-anime-game-launcher
// Copyright (C) 2021 - 2026  Nikita Podvirnyi <krypt0nn@dawn.wine>
// Copyright (C) 2025 - 2026  @JohnTheCoolingFan <ivan8215145640@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::LazyLock;

use anime_launcher_sdk::VERSION as SDK_VERSION;
use anime_launcher_sdk::anime_game_core::VERSION as CORE_VERSION;

use crate::*;

pub static APP_VERSION_STRING: LazyLock<String> = LazyLock::new(|| {
    if crate::APP_DEBUG && !crate::APP_VERSION.contains('-') {
        format!("{}-dev", crate::APP_VERSION)
    } else {
        crate::APP_VERSION.to_string()
    }
});

#[derive(Debug, Clone, Copy)]
pub struct AboutDialog;

#[relm4::component(pub)]
impl SimpleComponent for AboutDialog {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        dialog = adw::AboutDialog {
            set_application_name: "An Anime Game Launcher",
            set_application_icon: APP_ID,

            set_website: "https://github.com/an-anime-team/an-anime-game-launcher",
            set_issue_url: "https://github.com/an-anime-team/an-anime-game-launcher/issues",

            set_license_type: gtk::License::Gpl30Only,
            set_version: &APP_VERSION_STRING,

            set_developers: &[
                "Nikita Podvirnyi https://github.com/krypt0nn",
                "@JohnTheCoolingFan https://github.com/JohnTheCoolingFan"
            ],

            add_credit_section: (Some("An Anime Team"), &[
                "Nikita Podvirnyi https://github.com/krypt0nn",
                "Marie Piontek https://github.com/Mar0xy",
                "Luna Neff  https://github.com/lunaneff",
                "Renaud Lepage https://github.com/cybik",
                "Soham Nandy https://github.com/natimerry",
                "@mkrsym1 https://github.com/mkrsym1",
                "@JohnTheCoolingFan https://github.com/JohnTheCoolingFan"
            ]),

            set_artists: &[
                "@nightany https://pinterest.com/pin/356206651788051017"
            ],

            set_translator_credits: &[
                "Русский, English — Nikita Podvirnyi https://github.com/krypt0nn",
                "Deutsch — Marie Piontek https://github.com/Mar0xy",
                "Deutsch — @caem",
                "Français — @zeGolem https://github.com/zeGolem",
                "Español — Lautaro Garavano https://github.com/Rattlehead15",
                "Português — @kafushy",
                "Italiano — @QuazarOmega https://github.com/quazar-omega",
                "Türkçe — @Kaozix https://github.com/Kaozix1776",
                "Türkçe — Kayra Nachfolger https://github.com/kayranachfolger",
                "Türkçe — Berke Güzel https://github.com/wenekar",
                "Polski — Dominik Opyd https://github.com/oritwoen",
                // Hungarian?
                "Svenska — Jakob Fridesjö https://github.com/jakobfridesjo",
                "Nederlands — @everlyy https://github.com/everlyy",
                "简体中文 — Caibin Chen https://github.com/tigersoldier",
                "日本語 — @zozonteq https://github.com/zozonteq",
                "한국어 — @project-dy https://github.com/project-dy",
                "Indonesia — @yumekarisu https://github.com/yumekarisu",
                "Tiếng Việt — Nguyễn Hữu Chánh https://github.com/Chanhnh",
                "Tiếng Việt — OHaiiBuzzle https://github.com/ohaiibuzzle",
                "Українська — Іван Потієнко https://github.com/xxanqw",
                "ไทย — @vbrabandt https://github.com/vbrabandt",
                "Čeština — @panmourovaty https://github.com/panmourovaty"
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

            set_release_notes_version: &APP_VERSION_STRING,
            set_release_notes: r#"
                <p>Added</p>

                <ul>
                    <li>Added NTSync as a wine sync method (cosmetic/no-op, NTSync is used automatically on supported wine versions)</li>
                    <li>Added command-line arguments help text</li>
                </ul>

                <p>Fixed</p>

                <ul>
                    <li>Fixed %temp% and %game% command keywords being mixed up</li>
                </ul>

                <p>Changed</p>

                <ul>
                    <li>Improved readability of progress bar text</li>
                    <li>Updated french locale</li>
                </ul>
            "#
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing about dialog");

        let model = Self;
        let widgets = view_output!();

        ComponentParts {
            model,
            widgets
        }
    }
}
