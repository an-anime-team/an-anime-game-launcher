use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::sessions::SessionsExt;
use anime_launcher_sdk::genshin::sessions::Sessions;

use super::main::PreferencesAppMsg;

use crate::i18n::tr;
use crate::*;

#[derive(Debug)]
struct GameSession {
    name: String,
    description: Option<String>
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
            set_title: &self.name,

            set_subtitle: match &self.description {
                Some(description) => description.as_str(),
                None => ""
            },

            add_suffix = &gtk::Button {
                set_icon_name: "view-refresh-symbolic-symbolic",
                add_css_class: "flat",

                set_tooltip_text: Some(&tr("update-session")),

                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(GameAppMsg::UpdateSession(index.clone()));
                }
            },

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",

                set_tooltip_text: Some(&tr("delete-session")),

                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(GameAppMsg::RemoveSession(index.clone()));
                }
            }
        }
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        init
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }
}

pub struct GameApp {
    sessions: AsyncFactoryVecDeque<GameSession>,

    sessions_names: Vec<String>,

    sessions_combo: adw::ComboRow,
    session_name_entry: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum GameAppMsg {
    AddSession,
    UpdateSession(DynamicIndex),
    RemoveSession(DynamicIndex),
    SetCurrent(u32),
    UpdateCombo
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GameApp {
    type Init = ();
    type Input = GameAppMsg;
    type Output = PreferencesAppMsg;

    view! {
        adw::PreferencesPage {
            set_title: &tr("game"),
            set_icon_name: Some("applications-games-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr("game-sessions"),

                #[local_ref]
                sessions_combo -> adw::ComboRow {
                    set_title: &tr("active-sessions"),
                    set_subtitle: &tr("active-session-description"),

                    connect_selected_notify[sender] => move |row| {
                        if is_ready() {
                            sender.input(GameAppMsg::SetCurrent(row.selected()));
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                #[local_ref]
                session_name_entry -> adw::EntryRow {
                    set_title: &tr("name"),

                    add_suffix = &gtk::Button {
                        set_icon_name: "list-add-symbolic",
                        add_css_class: "flat",

                        set_valign: gtk::Align::Center,
    
                        connect_clicked => GameAppMsg::AddSession
                    }
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
        tracing::info!("Initializing game settings");

        let mut model = Self {
            sessions: AsyncFactoryVecDeque::new(adw::PreferencesGroup::new(), sender.input_sender()),

            sessions_names: Vec::new(),

            sessions_combo: adw::ComboRow::new(),
            session_name_entry: adw::EntryRow::new()
        };

        for (name, _) in Sessions::list().unwrap_or_default() {
            model.sessions.guard().push_back(GameSession {
                name: name.clone(),
                description: None
            });
        }

        let sessions = model.sessions.widget();

        let sessions_combo = &model.sessions_combo;
        let session_name_entry = &model.session_name_entry;

        let widgets = view_output!();

        sender.input(GameAppMsg::UpdateCombo);

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            GameAppMsg::AddSession => {
                let name = self.session_name_entry.text().trim().to_string();

                if !name.is_empty() {
                    if let Ok(config) = Config::get() {
                        self.session_name_entry.set_text("");

                        match Sessions::update(name.clone(), config.get_wine_prefix_path()) {
                            Ok(()) => {
                                self.sessions.guard().push_back(GameSession {
                                    name,
                                    description: None
                                });

                                sender.input(GameAppMsg::UpdateCombo);
                            }

                            #[allow(unused_must_use)]
                            Err(err) => {
                                sender.output(PreferencesAppMsg::Toast {
                                    title: tr("game-session-add-failed"),
                                    description: Some(err.to_string())
                                });
                            }
                        }
                    }
                }
            }

            GameAppMsg::UpdateSession(index) => {
                if let Some(session) = self.sessions.guard().get(index.current_index()) {
                    if let Ok(config) = Config::get() {
                        #[allow(unused_must_use)]
                        if let Err(err) = Sessions::update(session.name.clone(), config.get_wine_prefix_path()) {
                            sender.output(PreferencesAppMsg::Toast {
                                title: tr("game-session-update-failed"),
                                description: Some(err.to_string())
                            });
                        }
                    }
                }
            }

            GameAppMsg::RemoveSession(index) => {
                if let Some(session) = self.sessions.guard().get(index.current_index()) {
                    match Sessions::remove(&session.name) {
                        Ok(()) => sender.input(GameAppMsg::UpdateCombo),

                        #[allow(unused_must_use)]
                        Err(err) => {
                            sender.output(PreferencesAppMsg::Toast {
                                title: tr("game-session-remove-failed"),
                                description: Some(err.to_string())
                            });

                            return;
                        }
                    }
                }

                self.sessions.guard().remove(index.current_index());
            }

            GameAppMsg::SetCurrent(id) => {
                if let Some(name) = self.sessions_names.get(id as usize) {
                    if let Ok(config) = Config::get() {
                        #[allow(unused_must_use)]
                        if let Err(err) = Sessions::set_current(name.to_owned()) {
                            sender.output(PreferencesAppMsg::Toast {
                                title: tr("game-session-set-current-failed"),
                                description: Some(err.to_string())
                            });

                            // Prevent session applying
                            return;
                        }

                        #[allow(unused_must_use)]
                        if let Err(err) = Sessions::apply(name.to_owned(), config.get_wine_prefix_path()) {
                            sender.output(PreferencesAppMsg::Toast {
                                title: tr("game-session-apply-failed"),
                                description: Some(err.to_string())
                            });
                        }
                    }
                }
            }

            GameAppMsg::UpdateCombo => {
                let sessions = Sessions::get_sessions().unwrap_or_default();

                self.sessions_names = sessions.sessions.into_keys().collect::<Vec<String>>();

                let mut selected = 0;

                for (i, name) in self.sessions_names.iter().enumerate() {
                    if sessions.current.as_ref() == Some(name) {
                        selected = i as u32;
                    }
                }

                self.sessions_combo.set_model(Some(&gtk::StringList::new(&self.sessions_names.iter().map(|name: &String| name.as_str()).collect::<Vec<&str>>())));
                self.sessions_combo.set_selected(selected);
            }
        }
    }
}
