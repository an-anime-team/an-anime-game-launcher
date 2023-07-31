use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::sessions::SessionsExt;
use anime_launcher_sdk::genshin::sessions::Sessions;

use crate::*;

use super::EnhancementsAppMsg;

#[derive(Debug)]
struct GameSession {
    name: String,
    description: Option<String>,
    check_button: gtk::CheckButton
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

            // Looks weird but yes
            add_prefix = &self.check_button.clone(),

            add_suffix = &gtk::Button {
                set_icon_name: "view-refresh-symbolic",
                add_css_class: "flat",

                set_tooltip_text: Some(&tr!("update-session")),

                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(GamePageMsg::UpdateSession(index.current_index()));
                }
            },

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",

                set_tooltip_text: Some(&tr!("delete-session")),

                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(GamePageMsg::RemoveSession(index.current_index()));
                }
            },

            set_activatable: true,

            connect_activated[sender, index] => move |_| {
                sender.output(GamePageMsg::SetCurrent(index.current_index()));
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

    sessions_root_widget: gtk::CheckButton,
    session_name_entry: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum GamePageMsg {
    AddSession,
    UpdateSession(usize),
    RemoveSession(usize),
    SetCurrent(usize)
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
                    set_title: &tr!("game")
                },

                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",

                    connect_clicked[sender] => move |_| {
                        sender.output(EnhancementsAppMsg::OpenMainPage).unwrap();
                    }
                }
            },

            adw::PreferencesPage {
                set_title: &tr!("game"),
                set_icon_name: Some("applications-games-symbolic"),

                add = &adw::PreferencesGroup {
                    set_title: &tr!("game-sessions"),

                    #[local_ref]
                    session_name_entry -> adw::EntryRow {
                        set_title: &tr!("name"),

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

            sessions_root_widget: gtk::CheckButton::new(),
            session_name_entry: adw::EntryRow::new()
        };

        let current = Sessions::get_current().unwrap_or_default();

        for (name, _) in Sessions::list().unwrap_or_default() {
            let check_button = gtk::CheckButton::new();

            check_button.set_group(Some(&model.sessions_root_widget));

            if Some(&name) == current.as_ref() {
                check_button.set_active(true);
            }

            model.sessions.guard().push_back(GameSession {
                name,
                description: None,
                check_button
            });
        }

        let sessions = model.sessions.widget();

        let session_name_entry = &model.session_name_entry;

        let widgets = view_output!();

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
                                let check_button = gtk::CheckButton::new();

                                check_button.set_group(Some(&self.sessions_root_widget));

                                self.sessions.guard().push_back(GameSession {
                                    name,
                                    description: None,
                                    check_button
                                });
                            }

                            Err(err) => {
                                sender.output(EnhancementsAppMsg::Toast {
                                    title: tr!("game-session-add-failed"),
                                    description: Some(err.to_string())
                                }).unwrap();
                            }
                        }
                    }
                }
            }

            GamePageMsg::UpdateSession(index) => {
                if let Some(session) = self.sessions.guard().get(index) {
                    if let Ok(config) = Config::get() {
                        if let Err(err) = Sessions::update(session.name.clone(), config.get_wine_prefix_path()) {
                            sender.output(EnhancementsAppMsg::Toast {
                                title: tr!("game-session-update-failed"),
                                description: Some(err.to_string())
                            }).unwrap();
                        }
                    }
                }
            }

            GamePageMsg::RemoveSession(index) => {
                if let Some(session) = self.sessions.guard().get(index) {
                    if let Err(err) = Sessions::remove(&session.name) {
                        sender.output(EnhancementsAppMsg::Toast {
                            title: tr!("game-session-remove-failed"),
                            description: Some(err.to_string())
                        }).unwrap();

                        return;
                    }
                }

                self.sessions.guard().remove(index);

                if !self.sessions.is_empty() {
                    sender.input(GamePageMsg::SetCurrent(0));
                }
            }

            GamePageMsg::SetCurrent(index) => {
                if let Some(session) = self.sessions.guard().get(index) {
                    if let Ok(config) = Config::get() {
                        if let Err(err) = Sessions::set_current(session.name.clone()) {
                            sender.output(EnhancementsAppMsg::Toast {
                                title: tr!("game-session-set-current-failed"),
                                description: Some(err.to_string())
                            }).unwrap();

                            // Prevent session applying
                            return;
                        }

                        if let Err(err) = Sessions::apply(session.name.clone(), config.get_wine_prefix_path()) {
                            sender.output(EnhancementsAppMsg::Toast {
                                title: tr!("game-session-apply-failed"),
                                description: Some(err.to_string())
                            }).unwrap();

                            // Prevent session activation
                            return;
                        }

                        session.check_button.set_active(true);
                    }
                }
            }
        }
    }
}
