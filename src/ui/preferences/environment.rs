use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::config;

use crate::i18n::tr;
use crate::*;

#[derive(Debug)]
struct Variable {
    key: String,
    value: String
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for Variable {
    type Init = (String, String);
    type Input = EnvironmentMsg;
    type Output = EnvironmentMsg;
    type CommandOutput = ();
    type ParentInput = EnvironmentMsg;
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
                    sender.input(EnvironmentMsg::Remove(index.clone()));
                }
            }
        }
    }

    fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
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

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        sender.output(msg);
    }
}

pub struct EnvironmentApp {
    variables: AsyncFactoryVecDeque<Variable>,

    name: gtk::Entry,
    value: gtk::Entry
}

#[derive(Debug, Clone)]
pub enum EnvironmentMsg {
    Add,
    Remove(DynamicIndex)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for EnvironmentApp {
    type Init = ();
    type Input = EnvironmentMsg;
    type Output = ();

    view! {
        adw::PreferencesPage {
            set_title: &tr("environment"),
            set_icon_name: Some("document-properties-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr("game-command"),
                set_description: Some(&tr("game-command-description")),

                adw::EntryRow {
                    set_title: "%command%",
                    set_text: CONFIG.game.command.as_ref().unwrap_or(&String::new()).trim(),

                    connect_changed => |entry| {
                        if let Ok(mut config) = config::get() {
                            let command = entry.text().trim().to_string();
            
                            config.game.command = if command.is_empty() {
                                None
                            } else {
                                Some(command)
                            };
            
                            config::update(config);
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("new-variable"),

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,

                    #[local_ref]
                    name -> gtk::Entry {
                        set_placeholder_text: Some(&tr("name"))
                    },
    
                    #[local_ref]
                    value -> gtk::Entry {
                        set_placeholder_text: Some(&tr("value")),
                        set_hexpand: true
                    }
                },

                gtk::Button {
                    set_label: &tr("add"),

                    set_margin_top: 8,
                    set_halign: gtk::Align::Start,

                    connect_clicked => EnvironmentMsg::Add
                }
            },

            #[local_ref]
            add = variables -> adw::PreferencesGroup {}
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing environment settings");

        let mut model = Self {
            variables: AsyncFactoryVecDeque::new(adw::PreferencesGroup::new(), sender.input_sender()),

            name: gtk::Entry::new(),
            value: gtk::Entry::new()
        };

        for (name, value) in &CONFIG.game.environment {
            model.variables.guard().push_back((name.trim().to_string(), value.trim().to_string()));
        }

        let variables = model.variables.widget();

        let name = &model.name;
        let value = &model.value;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            EnvironmentMsg::Add => {
                if let Ok(mut config) = config::get() {
                    let name = self.name.text().trim().to_string();
                    let value = self.value.text().trim().to_string();

                    config.game.environment.insert(name.clone(), value.clone());

                    config::update(config);

                    self.variables.guard().push_back((name, value));
                }
            }

            EnvironmentMsg::Remove(index) => {
                if let Ok(mut config) = config::get() {
                    if let Some(var) = self.variables.guard().get(index.current_index()) {
                        config.game.environment.remove(&var.key);

                        config::update(config);
                    }

                    self.variables.guard().remove(index.current_index());
                }
            }
        }
    }
}
