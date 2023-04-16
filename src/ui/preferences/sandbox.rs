use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::is_available;

use crate::i18n::tr;
use crate::*;

#[derive(Debug)]
struct PrivateDirectory {
    path: String
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for PrivateDirectory {
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
                    sender.input(SandboxAppMsg::RemovePrivate(index.clone()));
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

#[derive(Debug)]
struct SharedDirectory {
    mount_from: String,
    mount_to: String
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for SharedDirectory {
    type Init = (String, String);
    type Input = SandboxAppMsg;
    type Output = SandboxAppMsg;
    type CommandOutput = ();
    type ParentInput = SandboxAppMsg;
    type ParentWidget = adw::PreferencesGroup;

    view! {
        root = adw::ActionRow {
            set_title: &self.mount_to,
            set_subtitle: &self.mount_from,

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.input(SandboxAppMsg::RemoveShared(index.clone()));
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
            mount_from: init.0,
            mount_to: init.1
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        sender.output(msg);
    }
}

pub struct SandboxApp {
    private_paths: AsyncFactoryVecDeque<PrivateDirectory>,
    shared_paths: AsyncFactoryVecDeque<SharedDirectory>,

    private_path_entry: adw::EntryRow,

    shared_path_from_entry: adw::EntryRow,
    shared_path_to_entry: adw::EntryRow,
    read_only_switch: gtk::Switch
}

#[derive(Debug, Clone)]
pub enum SandboxAppMsg {
    AddPrivate,
    RemovePrivate(DynamicIndex),

    AddShared,
    RemoveShared(DynamicIndex)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for SandboxApp {
    type Init = ();
    type Input = SandboxAppMsg;
    type Output = ();

    view! {
        adw::PreferencesPage {
            set_title: &tr("sandbox"),
            set_icon_name: Some("folder-symbolic"),

            set_sensitive: is_available("bwrap"),

            add = &adw::PreferencesGroup {
                set_title: &tr("sandbox"),
                set_description: Some(&tr("sandbox-description")),

                adw::ActionRow {
                    set_title: &tr("enable-sandboxing"),
                    set_subtitle: &tr("enable-sandboxing-description"),

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
                    set_title: &tr("hide-home-directory"),
                    set_subtitle: &tr("hide-home-directory-description"),

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
                },

                adw::EntryRow {
                    set_title: &tr("hostname"),
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("private-directories"),
                set_description: Some(&tr("private-directories-description")),

                #[local_ref]
                private_path_entry -> adw::EntryRow {
                    set_title: &tr("path")
                },

                gtk::Button {
                    set_label: &tr("add"),
                    add_css_class: "pill",

                    set_margin_top: 8,
                    set_halign: gtk::Align::Start,

                    connect_clicked => SandboxAppMsg::AddPrivate
                }
            },

            #[local_ref]
            add = private_paths -> adw::PreferencesGroup {},

            add = &adw::PreferencesGroup {
                set_title: &tr("shared-directories"),
                set_description: Some(&tr("shared-directories-description")),

                #[local_ref]
                shared_path_from_entry -> adw::EntryRow {
                    set_title: &tr("original-path")
                },

                #[local_ref]
                shared_path_to_entry -> adw::EntryRow {
                    set_title: &tr("new-path")
                },

                adw::ActionRow {
                    set_title: &tr("read-only"),
                    set_subtitle: &tr("read-only-description"),

                    #[local_ref]
                    add_suffix = read_only_switch -> gtk::Switch {
                        set_valign: gtk::Align::Center
                    }
                },

                gtk::Button {
                    set_label: &tr("add"),
                    add_css_class: "pill",

                    set_margin_top: 8,
                    set_halign: gtk::Align::Start,

                    connect_clicked => SandboxAppMsg::AddShared
                }
            },

            #[local_ref]
            add = shared_paths -> adw::PreferencesGroup {}
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing environment settings");

        let mut model = Self {
            private_paths: AsyncFactoryVecDeque::new(adw::PreferencesGroup::new(), sender.input_sender()),
            shared_paths: AsyncFactoryVecDeque::new(adw::PreferencesGroup::new(), sender.input_sender()),

            private_path_entry: adw::EntryRow::new(),

            shared_path_from_entry: adw::EntryRow::new(),
            shared_path_to_entry: adw::EntryRow::new(),
            read_only_switch: gtk::Switch::new()
        };

        for path in &CONFIG.sandbox.private {
            model.private_paths.guard().push_back(path.trim().to_string());
        }

        for (from, to) in &CONFIG.sandbox.mounts.read_only {
            model.shared_paths.guard().push_back((
                from.trim().to_string(),
                format!("[read-only] {}", to.trim())
            ));
        }

        for (from, to) in &CONFIG.sandbox.mounts.bind {
            model.shared_paths.guard().push_back((
                from.trim().to_string(),
                to.trim().to_string()
            ));
        }

        let private_paths = model.private_paths.widget();
        let shared_paths = model.shared_paths.widget();

        let private_path_entry = &model.private_path_entry;

        let shared_path_from_entry = &model.shared_path_from_entry;
        let shared_path_to_entry = &model.shared_path_to_entry;
        let read_only_switch = &model.read_only_switch;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            SandboxAppMsg::AddPrivate => {
                if let Ok(mut config) = Config::get() {
                    let path = self.private_path_entry.text().trim().to_string();

                    if !path.is_empty() {
                        config.sandbox.private.push(path.clone());

                        Config::update(config);

                        self.private_paths.guard().push_back(path);
                    }
                }
            }

            SandboxAppMsg::RemovePrivate(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.private_paths.guard().get(index.current_index()) {
                        config.sandbox.private.retain(|item| item != &var.path);

                        Config::update(config);
                    }

                    self.private_paths.guard().remove(index.current_index());
                }
            },

            SandboxAppMsg::AddShared => {
                if let Ok(mut config) = Config::get() {
                    let from = self.shared_path_from_entry.text().trim().to_string();
                    let to = self.shared_path_to_entry.text().trim().to_string();

                    let read_only = self.read_only_switch.state();

                    if !from.is_empty() && !to.is_empty() {
                        if read_only {
                            config.sandbox.mounts.read_only.insert(from.clone(), to.clone());
                        } else {
                            config.sandbox.mounts.bind.insert(from.clone(), to.clone());
                        }

                        Config::update(config);

                        self.shared_paths.guard().push_back((
                            from,
                            if read_only {
                                format!("[read-only] {}", to)
                            } else {
                                to
                            }
                        ));
                    }
                }
            }

            SandboxAppMsg::RemoveShared(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.shared_paths.guard().get(index.current_index()) {
                        config.sandbox.mounts.read_only.remove(&var.mount_from);
                        config.sandbox.mounts.bind.remove(&var.mount_from);

                        Config::update(config);
                    }

                    self.shared_paths.guard().remove(index.current_index());
                }
            }
        }
    }
}
