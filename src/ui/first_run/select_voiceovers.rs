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

use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::Config;

use crate::*;

use super::main::*;

pub struct SelectVoiceoversApp {
    english: gtk::Switch,
    japanese: gtk::Switch,
    korean: gtk::Switch,
    chinese: gtk::Switch
}

#[derive(Debug, Clone)]
pub enum SelectVoiceoversAppMsg {
    Continue,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for SelectVoiceoversApp {
    type Init = ();
    type Input = SelectVoiceoversAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: &i18n!("select-voice-packages"),
                    add_css_class: "title-1"
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                adw::ActionRow {
                    set_title: &i18n!("english"),

                    #[local_ref]
                    add_suffix = english -> gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_active: true
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("japanese"),

                    #[local_ref]
                    add_suffix = japanese -> gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("korean"),

                    #[local_ref]
                    add_suffix = korean -> gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                },

                adw::ActionRow {
                    set_title: &i18n!("chinese"),

                    #[local_ref]
                    add_suffix = chinese -> gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
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

                        connect_clicked => SelectVoiceoversAppMsg::Continue
                    },

                    gtk::Button {
                        set_label: &i18n!("exit"),
                        add_css_class: "pill",

                        connect_clicked => SelectVoiceoversAppMsg::Exit
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            english: gtk::Switch::new(),
            japanese: gtk::Switch::new(),
            korean: gtk::Switch::new(),
            chinese: gtk::Switch::new()
        };

        let english  = &model.english;
        let japanese = &model.japanese;
        let korean   = &model.korean;
        let chinese  = &model.chinese;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            #[allow(unused_must_use)]
            SelectVoiceoversAppMsg::Continue => {
                match self.update_config() {
                    Ok(_) => sender.output(Self::Output::ScrollToDownloadComponents),

                    Err(err) => sender.output(Self::Output::Toast {
                        title: i18n!("config-update-error"),
                        description: Some(err.to_string())
                    })
                };
            }

            SelectVoiceoversAppMsg::Exit => relm4::main_application().quit()
        }
    }
}

impl SelectVoiceoversApp {
    pub fn update_config(&self) -> anyhow::Result<()> {
        let mut config = Config::get()?;

        config.game.voices = Vec::new();

        if self.english.state() {
            config.game.voices.push(String::from("en-us"));
        }

        if self.japanese.state() {
            config.game.voices.push(String::from("ja-jp"));
        }

        if self.korean.state() {
            config.game.voices.push(String::from("ko-kr"));
        }

        if self.chinese.state() {
            config.game.voices.push(String::from("zh-cn"));
        }

        Config::update_raw(config)
    }
}
