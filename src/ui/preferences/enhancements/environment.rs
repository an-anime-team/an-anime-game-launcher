use relm4::prelude::*;
use relm4::factory::*;

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
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: &tr!("environment")
                },

                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",

                    connect_clicked[sender] => move |_| {
                        sender.output(EnhancementsAppMsg::OpenMainPage).unwrap();
                    }
                }
            },

            adw::PreferencesPage {
                set_title: &tr!("environment"),
                set_icon_name: Some("document-properties-symbolic"),

                add = &adw::PreferencesGroup {
                    set_title: &tr!("game-command"),
                    set_description: Some(&tr!("game-command-description")),

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
                    set_title: &tr!("new-variable"),

                    #[wrap(Some)]
                    set_header_suffix = &gtk::Button {
                        add_css_class: "flat",

                        set_valign: gtk::Align::Center,

                        adw::ButtonContent {
                            set_icon_name: "list-add-symbolic",
                            set_label: &tr!("add")
                        },

                        connect_clicked => EnvironmentPageMsg::Add
                    },

                    #[local_ref]
                    name_entry -> adw::EntryRow {
                        set_title: &tr!("name")
                    },

                    #[local_ref]
                    value_entry -> adw::EntryRow {
                        set_title: &tr!("value")
                    }
                },

                #[local_ref]
                add = variables -> adw::PreferencesGroup {}
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

                if !name.is_empty() && !value.is_empty() {
                    if let Ok(mut config) = Config::get() {
                        self.name_entry.set_text("");
                        self.value_entry.set_text("");

                        config.game.environment.insert(name.clone(), value.clone());

                        Config::update(config);

                        self.variables.guard().push_back((name, value));
                    }
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
