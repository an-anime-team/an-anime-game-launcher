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

use crate::*;

use super::EnhancementsAppMsg;

#[derive(Debug)]
struct Variable {
    key: String,
    value: String
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for Variable {
    type Init = (String, String);
    type Input = EnvironmentPageMsg;
    type Output = EnvironmentPageMsg;
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        root = adw::ActionRow {
            set_title: &self.key,
            set_subtitle: &self.value,

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(EnvironmentPageMsg::Remove(index.clone()))
                        .unwrap();
                }
            }
        }
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            key: init.0,
            value: init.1
        }
    }
}

pub struct EnvironmentPage {
    variables: AsyncFactoryVecDeque<Variable>,

    name_entry: adw::EntryRow,
    value_entry: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum EnvironmentPageMsg {
    Add,
    Remove(DynamicIndex)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for EnvironmentPage {
    type Init = ();
    type Input = EnvironmentPageMsg;
    type Output = EnhancementsAppMsg;

    view! {
        adw::NavigationPage {
            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: &i18n!("environment")
                    }
                },

                adw::PreferencesPage {
                    set_title: &i18n!("environment"),
                    set_icon_name: Some("document-properties-symbolic"),

                    add = &adw::PreferencesGroup {
                        set_title: &i18n!("game-command"),
                        set_description: Some(&i18n!("game-command-description")),

                        adw::EntryRow {
                            set_title: "%command%",
                            set_text: CONFIG.game.command.as_ref().unwrap_or(&String::new()).trim(),

                            connect_changed => |entry| {
                                if let Ok(mut config) = Config::get() {
                                    let command = entry.text().trim().to_string();

                                    config.game.command = if command.is_empty() {
                                        None
                                    } else {
                                        Some(command)
                                    };

                                    Config::update(config);
                                }
                            }
                        }
                    },

                    add = &adw::PreferencesGroup {
                        set_title: &i18n!("new-variable"),

                        #[wrap(Some)]
                        set_header_suffix = &gtk::Button {
                            add_css_class: "flat",

                            set_valign: gtk::Align::Center,

                            adw::ButtonContent {
                                set_icon_name: "list-add-symbolic",
                                set_label: &i18n!("add")
                            },

                            connect_clicked => EnvironmentPageMsg::Add
                        },

                        #[local_ref]
                        name_entry -> adw::EntryRow {
                            set_title: &i18n!("name")
                        },

                        #[local_ref]
                        value_entry -> adw::EntryRow {
                            set_title: &i18n!("value")
                        }
                    },

                    #[local_ref]
                    add = variables -> adw::PreferencesGroup {}
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing environment settings");

        let mut model = Self {
            variables: AsyncFactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), std::convert::identity),

            name_entry: adw::EntryRow::new(),
            value_entry: adw::EntryRow::new()
        };

        for (name, value) in &CONFIG.game.environment {
            model.variables.guard().push_back((name.trim().to_string(), value.trim().to_string()));
        }

        let variables = model.variables.widget();

        let name_entry = &model.name_entry;
        let value_entry = &model.value_entry;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            EnvironmentPageMsg::Add => {
                let name = self.name_entry.text().trim().to_string();
                let value = self.value_entry.text().trim().to_string();

                if !name.is_empty()
                    && !value.is_empty()
                    && let Ok(mut config) = Config::get()
                {
                    self.name_entry.set_text("");
                    self.value_entry.set_text("");

                    config.game.environment.insert(name.clone(), value.clone());

                    Config::update(config);

                    self.variables.guard().push_back((name, value));
                }
            }

            EnvironmentPageMsg::Remove(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.variables.guard().get(index.current_index()) {
                        config.game.environment.remove(&var.key);

                        Config::update(config);
                    }

                    self.variables.guard().remove(index.current_index());
                }
            }
        }
    }
}
