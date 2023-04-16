use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::is_available;

use crate::i18n::tr;
use crate::*;

#[derive(Debug)]
struct Directory {
    path: String
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for Directory {
    type Init = String;
    type Input = SandboxAppMsg;
    type Output = SandboxAppMsg;
    type CommandOutput = ();
    type ParentInput = SandboxAppMsg;
    type ParentWidget = adw::PreferencesGroup;

    view! {
        root = adw::ActionRow {
            set_title: &self.path,

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.input(SandboxAppMsg::Remove(index.clone()));
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
            path: init
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        sender.output(msg);
    }
}

pub struct SandboxApp {
    directories: AsyncFactoryVecDeque<Directory>,

    sandboxing_path: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum SandboxAppMsg {
    Add,
    Remove(DynamicIndex)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for SandboxApp {
    type Init = ();
    type Input = SandboxAppMsg;
    type Output = ();

    view! {
        adw::PreferencesPage {
            set_title: "Sandbox",
            set_icon_name: Some("folder-symbolic"),

            set_sensitive: is_available("bwrap"),

            add = &adw::PreferencesGroup {
                set_title: "Sandbox",
                set_description: Some("Run the game in isolated environment, preventing it from accessing your personal data"),

                adw::ActionRow {
                    set_title: "Enable sandboxing",
                    set_subtitle: "Run the game in read-only copy of your root filesystem",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.sandbox.enabled,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.sandbox.enabled = switch.state();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                },

                adw::ActionRow {
                    set_title: "Hide home directory",
                    set_subtitle: "Isolate your /home, /var/home/{username}, and $HOME folders from the game",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        set_state: CONFIG.sandbox.isolate_home,

                        connect_state_notify => |switch| {
                            if is_ready() {
                                if let Ok(mut config) = Config::get() {
                                    config.sandbox.isolate_home = switch.state();

                                    Config::update(config);
                                }
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: "Sandboxed directories",
                set_description: Some("These folders will be replaced by in-memory filesystem (tmpfs), and their original content will not be available to sandboxed game"),

                #[local_ref]
                sandboxing_path -> adw::EntryRow {
                    set_title: "Path"
                },

                gtk::Button {
                    set_label: &tr("add"),
                    add_css_class: "pill",

                    set_margin_top: 8,
                    set_halign: gtk::Align::Start,

                    connect_clicked => SandboxAppMsg::Add
                }
            },

            #[local_ref]
            add = directories -> adw::PreferencesGroup {}
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing environment settings");

        let mut model = Self {
            directories: AsyncFactoryVecDeque::new(adw::PreferencesGroup::new(), sender.input_sender()),

            sandboxing_path: adw::EntryRow::new()
        };

        for path in &CONFIG.sandbox.private {
            model.directories.guard().push_back(path.trim().to_string());
        }

        let directories = model.directories.widget();
        let sandboxing_path = &model.sandboxing_path;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            SandboxAppMsg::Add => {
                if let Ok(mut config) = Config::get() {
                    let path = self.sandboxing_path.text().trim().to_string();

                    if !path.is_empty() {
                        config.sandbox.private.push(path.clone());

                        Config::update(config);

                        self.directories.guard().push_back(path);
                    }
                }
            }

            SandboxAppMsg::Remove(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.directories.guard().get(index.current_index()) {
                        config.sandbox.private.retain(|item| item != &var.path);

                        Config::update(config);
                    }

                    self.directories.guard().remove(index.current_index());
                }
            }
        }
    }
}
