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

use adw::prelude::*;

use anime_launcher_sdk::is_available;

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
                    set_resource: Some(&format!("{APP_RESOURCE_PATH}/icons/scalable/apps/{APP_ID}.png")),
                    set_height_request: 128
                },

                gtk::Label {
                    set_label: "An Anime Game Launcher",
                    set_margin_top: 32,
                    add_css_class: "title-1"
                },

                gtk::Label {
                    set_label: &i18n!("welcome-page-message"),

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
                        set_label: &i18n!("continue"),
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
        _sender: AsyncComponentSender<Self>
    ) -> AsyncComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();

        AsyncComponentParts {
            model,
            widgets
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            #[allow(unused_must_use)]
            WelcomeAppMsg::Continue => {
                let installed = is_available("git") && (is_available("7z") || is_available("7za"));

                if installed {
                    sender.output(Self::Output::ScrollToDefaultPaths);
                } else {
                    sender.output(Self::Output::ScrollToDependencies);
                }
            }
        }
    }
}
