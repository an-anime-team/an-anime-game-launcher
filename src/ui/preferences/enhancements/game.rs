use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::sessions::SessionsExt;
use anime_launcher_sdk::genshin::sessions::Sessions;

use super::EnhancementsAppMsg;

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
    type Input = GamePageMsg;
    type Output = GamePageMsg;
    type CommandOutput = ();
    type ParentInput = GamePageMsg;
    type ParentWidget = adw::PreferencesGroup;

    view! {
        root = adw::ActionRow {
            set_title: &self.name,

            set_subtitle: match &self.description {
                Some(description) => description.as_str(),
                None => ""
            },

            add_suffix = &gtk::Button {
                set_icon_name: "view-refresh-symbolic",
                add_css_class: "flat",

                set_tooltip_text: Some(&tr("update-session")),

                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(GamePageMsg::UpdateSession(index.clone()));
                }
            },

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",

                set_tooltip_text: Some(&tr("delete-session")),

                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(GamePageMsg::RemoveSession(index.clone()));
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

pub struct GamePage {
    sessions: AsyncFactoryVecDeque<GameSession>,

    sessions_names: Vec<String>,

    sessions_combo: adw::ComboRow,
    session_name_entry: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum GamePageMsg {
    AddSession,
    UpdateSession(DynamicIndex),
    RemoveSession(DynamicIndex),
    SetCurrent(u32),
    UpdateCombo
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for GamePage {
    type Init = ();
    type Input = GamePageMsg;
    type Output = EnhancementsAppMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: &tr("game")
                },

                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",

                    connect_clicked[sender] => move |_| {
                        sender.output(EnhancementsAppMsg::OpenMainPage).unwrap();
                    }
                }
            },

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
                                sender.input(GamePageMsg::SetCurrent(row.selected()));
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
        
                            connect_clicked => GamePageMsg::AddSession
                        }
                    }
                },
    
                #[local_ref]
                add = sessions -> adw::PreferencesGroup {},
            }
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

        sender.input(GamePageMsg::UpdateCombo);

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            GamePageMsg::AddSession => {
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

                                sender.input(GamePageMsg::UpdateCombo);
                            }

                            Err(err) => {
                                sender.output(EnhancementsAppMsg::Toast {
                                    title: tr("game-session-add-failed"),
                                    description: Some(err.to_string())
                                }).unwrap();
                            }
                        }
                    }
                }
            }

            GamePageMsg::UpdateSession(index) => {
                if let Some(session) = self.sessions.guard().get(index.current_index()) {
                    if let Ok(config) = Config::get() {
                        if let Err(err) = Sessions::update(session.name.clone(), config.get_wine_prefix_path()) {
                            sender.output(EnhancementsAppMsg::Toast {
                                title: tr("game-session-update-failed"),
                                description: Some(err.to_string())
                            }).unwrap();
                        }
                    }
                }
            }

            GamePageMsg::RemoveSession(index) => {
                if let Some(session) = self.sessions.guard().get(index.current_index()) {
                    match Sessions::remove(&session.name) {
                        Ok(()) => sender.input(GamePageMsg::UpdateCombo),

                        Err(err) => {
                            sender.output(EnhancementsAppMsg::Toast {
                                title: tr("game-session-remove-failed"),
                                description: Some(err.to_string())
                            }).unwrap();

                            return;
                        }
                    }
                }

                self.sessions.guard().remove(index.current_index());
            }

            GamePageMsg::SetCurrent(id) => {
                if let Some(name) = self.sessions_names.get(id as usize) {
                    if let Ok(config) = Config::get() {
                        if let Err(err) = Sessions::set_current(name.to_owned()) {
                            sender.output(EnhancementsAppMsg::Toast {
                                title: tr("game-session-set-current-failed"),
                                description: Some(err.to_string())
                            }).unwrap();

                            // Prevent session applying
                            return;
                        }

                        if let Err(err) = Sessions::apply(name.to_owned(), config.get_wine_prefix_path()) {
                            sender.output(EnhancementsAppMsg::Toast {
                                title: tr("game-session-apply-failed"),
                                description: Some(err.to_string())
                            }).unwrap();
                        }
                    }
                }
            }

            GamePageMsg::UpdateCombo => {
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
