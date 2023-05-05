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
                    sender.input(GameAppMsg::RemoveSession(index.clone()));
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
    sessions: AsyncFactoryVecDeque<GameSession>,

    session_name_entry: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum GameAppMsg {
    AddSession,
    RemoveSession(DynamicIndex)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GameApp {
    type Init = ();
    type Input = GameAppMsg;
    type Output = ();

    view! {
        adw::PreferencesPage {
            set_title: "Game",
            set_icon_name: Some("document-properties-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: "Saved sessions",

                #[local_ref]
                session_name_entry -> adw::EntryRow {
                    set_title: &tr("name")
                },

                gtk::Button {
                    set_label: &tr("add"),
                    add_css_class: "pill",

                    set_margin_top: 8,
                    set_halign: gtk::Align::Start,

                    connect_clicked => GameAppMsg::AddSession
                }
            },

            #[local_ref]
            add = sessions -> adw::PreferencesGroup {},
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing environment settings");

        let mut model = Self {
            sessions: AsyncFactoryVecDeque::new(adw::PreferencesGroup::new(), sender.input_sender()),

            session_name_entry: adw::EntryRow::new()
        };

        /*for (name, value) in &CONFIG.game.environment {
            model.variables.guard().push_back();
        }*/

        let sessions = model.sessions.widget();

        let session_name_entry = &model.session_name_entry;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            GameAppMsg::AddSession => {
                let name = self.session_name_entry.text().trim().to_string();

                if !name.is_empty() {
                    self.session_name_entry.set_text("");

                    self.sessions.guard().push_back(GameSession {
                        title: name,
                        description: None,
                        id: 0
                    });
                }
            }

            GameAppMsg::RemoveSession(index) => {
                if let Some(var) = self.sessions.guard().get(index.current_index()) {
                    // ..
                }

                self.sessions.guard().remove(index.current_index());
            }
        }
    }
}
