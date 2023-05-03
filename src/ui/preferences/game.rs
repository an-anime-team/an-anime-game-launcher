use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use crate::i18n::tr;
use crate::*;

#[derive(Debug)]
struct GameSession {
    title: String,
    description: Option<String>,
    id: usize
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for GameSession {
    type Init = GameSession;
    type Input = GameAppMsg;
    type Output = GameAppMsg;
    type CommandOutput = ();
    type ParentInput = GameAppMsg;
    type ParentWidget = adw::PreferencesGroup;

    view! {
        root = adw::ActionRow {
            set_title: &self.title,

            set_subtitle: match &self.description {
                Some(description) => description.as_str(),
                None => ""
            },

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.input(GameAppMsg::Remove(index.clone()));
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
        init
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        sender.output(msg);
    }
}

pub struct GameApp {
    variables: AsyncFactoryVecDeque<GameSession>,

    name_entry: adw::EntryRow,
    value_entry: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum GameAppMsg {
    Add,
    Remove(DynamicIndex)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GameApp {
    type Init = ();
    type Input = GameAppMsg;
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
                set_title: &tr("new-variable"),

                #[local_ref]
                name_entry -> adw::EntryRow {
                    set_title: &tr("name")
                },

                #[local_ref]
                value_entry -> adw::EntryRow {
                    set_title: &tr("value")
                },

                gtk::Button {
                    set_label: &tr("add"),
                    add_css_class: "pill",

                    set_margin_top: 8,
                    set_halign: gtk::Align::Start,

                    connect_clicked => GameAppMsg::Add
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

            name_entry: adw::EntryRow::new(),
            value_entry: adw::EntryRow::new()
        };

        /*for (name, value) in &CONFIG.game.environment {
            model.variables.guard().push_back();
        }*/

        let variables = model.variables.widget();

        let name_entry = &model.name_entry;
        let value_entry = &model.value_entry;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            GameAppMsg::Add => {
                if let Ok(mut config) = Config::get() {
                    let name = self.name_entry.text().trim().to_string();
                    let value = self.value_entry.text().trim().to_string();

                    if !name.is_empty() && !value.is_empty() {
                        self.name_entry.set_text("");
                        self.value_entry.set_text("");

                        config.game.environment.insert(name.clone(), value.clone());

                        Config::update(config);

                        // self.variables.guard().push_back((name, value));
                    }
                }
            }

            GameAppMsg::Remove(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.variables.guard().get(index.current_index()) {
                        // config.game.environment.remove(&var.key);

                        Config::update(config);
                    }

                    self.variables.guard().remove(index.current_index());
                }
            }
        }
    }
}
