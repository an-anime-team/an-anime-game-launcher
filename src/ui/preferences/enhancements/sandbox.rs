use relm4::prelude::*;
use relm4::factory::*;

use adw::prelude::*;

use anime_launcher_sdk::is_available;

use super::EnhancementsAppMsg;

use crate::*;

macro_rules! impl_directory {
    ($name:ident, $msg:expr) => {
        #[derive(Debug)]
        struct $name {
            from: String,
            to: Option<String>
        }

        #[relm4::factory(async)]
        impl AsyncFactoryComponent for $name {
            type Init = (String, Option<String>);
            type Input = SandboxPageMsg;
            type Output = SandboxPageMsg;
            type CommandOutput = ();
            type ParentWidget = adw::PreferencesGroup;

            view! {
                root = adw::ActionRow {
                    set_title: &self.from,
                    set_subtitle: match self.to.as_ref() {
                        Some(to) => to,
                        None => ""
                    },

                    add_suffix = &gtk::Button {
                        set_icon_name: "user-trash-symbolic",
                        add_css_class: "flat",
                        set_valign: gtk::Align::Center,

                        connect_clicked[sender, index] => move |_| {
                            sender.output($msg(index.clone()))
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
                    from: init.0,
                    to: init.1
                }
            }
        }
    }
}

impl_directory!(PrivateDirectory, SandboxPageMsg::RemovePrivate);
impl_directory!(SharedDirectory, SandboxPageMsg::RemoveShared);
impl_directory!(SymlinkPath, SandboxPageMsg::RemoveSymlink);

pub struct SandboxPage {
    private_paths: AsyncFactoryVecDeque<PrivateDirectory>,
    shared_paths: AsyncFactoryVecDeque<SharedDirectory>,
    symlink_paths: AsyncFactoryVecDeque<SymlinkPath>,

    private_path_entry: adw::EntryRow,

    shared_path_from_entry: adw::EntryRow,
    shared_path_to_entry: adw::EntryRow,
    read_only_switch: gtk::Switch,

    symlink_path_from_entry: adw::EntryRow,
    symlink_path_to_entry: adw::EntryRow
}

#[derive(Debug, Clone)]
pub enum SandboxPageMsg {
    AddPrivate,
    RemovePrivate(DynamicIndex),

    AddShared,
    RemoveShared(DynamicIndex),

    AddSymlink,
    RemoveSymlink(DynamicIndex)
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for SandboxPage {
    type Init = ();
    type Input = SandboxPageMsg;
    type Output = EnhancementsAppMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: &tr!("sandbox")
                },

                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",

                    connect_clicked[sender] => move |_| {
                        sender.output(EnhancementsAppMsg::OpenMainPage).unwrap();
                    }
                }
            },

            adw::PreferencesPage {
                set_title: &tr!("sandbox"),
                set_icon_name: Some("folder-symbolic"),

                set_sensitive: is_available("bwrap"),

                add = &adw::PreferencesGroup {
                    set_title: &tr!("sandbox"),
                    set_description: Some(&tr!("sandbox-description")),

                    adw::ActionRow {
                        set_title: &tr!("enable-sandboxing"),
                        set_subtitle: &tr!("enable-sandboxing-description"),

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
                        set_title: &tr!("hide-home-directory"),
                        set_subtitle: &tr!("hide-home-directory-description"),

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
                        set_title: &tr!("hostname"),
                        set_text: CONFIG.sandbox.hostname.as_ref().unwrap_or(&String::new()).trim(),

                        connect_changed => |entry| {
                            if let Ok(mut config) = Config::get() {
                                let command = entry.text().trim().to_string();

                                config.sandbox.hostname = if command.is_empty() {
                                    None
                                } else {
                                    Some(command)
                                };

                                Config::update(config);
                            }
                        }
                    },

                    adw::EntryRow {
                        set_title: &tr!("additional-arguments"),
                        set_text: CONFIG.sandbox.args.as_ref().unwrap_or(&String::new()).trim(),

                        connect_changed => |entry| {
                            if let Ok(mut config) = Config::get() {
                                let command = entry.text().trim().to_string();

                                config.sandbox.args = if command.is_empty() {
                                    None
                                } else {
                                    Some(command)
                                };

                                Config::update(config);
                            }
                        },

                        add_suffix = &gtk::Button {
                            set_icon_name: "dialog-information-symbolic",
                            add_css_class: "flat",

                            set_valign: gtk::Align::Center,

                            connect_clicked[sender] => move |_| {
                                if let Err(err) = open::that("https://man.archlinux.org/man/bwrap.1") {
                                    sender.output(EnhancementsAppMsg::Toast {
                                        title: tr!("documentation-url-open-failed"),
                                        description: Some(err.to_string())
                                    }).unwrap();
                                }
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: &tr!("private-directories"),
                    set_description: Some(&tr!("private-directories-description")),

                    #[local_ref]
                    private_path_entry -> adw::EntryRow {
                        set_title: &tr!("path"),

                        add_suffix = &gtk::Button {
                            set_icon_name: "list-add-symbolic",
                            add_css_class: "flat",

                            set_valign: gtk::Align::Center,
        
                            connect_clicked => SandboxPageMsg::AddPrivate
                        }
                    }
                },

                #[local_ref]
                add = private_paths -> adw::PreferencesGroup {},

                add = &adw::PreferencesGroup {
                    set_title: &tr!("shared-directories"),
                    set_description: Some(&tr!("shared-directories-description")),

                    #[wrap(Some)]
                    set_header_suffix = &gtk::Button {
                        add_css_class: "flat",

                        set_valign: gtk::Align::Center,

                        adw::ButtonContent {
                            set_icon_name: "list-add-symbolic",
                            set_label: &tr!("add")
                        },

                        connect_clicked => SandboxPageMsg::AddShared
                    },

                    #[local_ref]
                    shared_path_from_entry -> adw::EntryRow {
                        set_title: &tr!("original-path")
                    },

                    #[local_ref]
                    shared_path_to_entry -> adw::EntryRow {
                        set_title: &tr!("new-path")
                    },

                    adw::ActionRow {
                        set_title: &tr!("read-only"),
                        set_subtitle: &tr!("read-only-description"),

                        #[local_ref]
                        add_suffix = read_only_switch -> gtk::Switch {
                            set_valign: gtk::Align::Center
                        }
                    }
                },

                #[local_ref]
                add = shared_paths -> adw::PreferencesGroup {},

                add = &adw::PreferencesGroup {
                    set_title: &tr!("symlinks"),
                    set_description: Some(&tr!("symlinks-description")),

                    #[wrap(Some)]
                    set_header_suffix = &gtk::Button {
                        add_css_class: "flat",

                        set_valign: gtk::Align::Center,

                        adw::ButtonContent {
                            set_icon_name: "list-add-symbolic",
                            set_label: &tr!("add")
                        },

                        connect_clicked => SandboxPageMsg::AddSymlink
                    },

                    #[local_ref]
                    symlink_path_from_entry -> adw::EntryRow {
                        set_title: &tr!("original-path")
                    },

                    #[local_ref]
                    symlink_path_to_entry -> adw::EntryRow {
                        set_title: &tr!("new-path")
                    }
                },

                #[local_ref]
                add = symlink_paths -> adw::PreferencesGroup {}
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        tracing::info!("Initializing sandbox settings");

        let mut model = Self {
            private_paths: AsyncFactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), std::convert::identity),

            shared_paths: AsyncFactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), std::convert::identity),

            symlink_paths: AsyncFactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), std::convert::identity),

            private_path_entry: adw::EntryRow::new(),

            shared_path_from_entry: adw::EntryRow::new(),
            shared_path_to_entry: adw::EntryRow::new(),
            read_only_switch: gtk::Switch::new(),

            symlink_path_from_entry: adw::EntryRow::new(),
            symlink_path_to_entry: adw::EntryRow::new()
        };

        for path in &CONFIG.sandbox.private {
            model.private_paths.guard().push_back((path.trim().to_string(), None));
        }

        for (from, to) in &CONFIG.sandbox.mounts.read_only {
            model.shared_paths.guard().push_back((
                from.trim().to_string(),
                Some(format!("[read-only] {}", to.trim()))
            ));
        }

        for (from, to) in &CONFIG.sandbox.mounts.bind {
            model.shared_paths.guard().push_back((
                from.trim().to_string(),
                Some(to.trim().to_string())
            ));
        }

        for (from, to) in &CONFIG.sandbox.mounts.symlinks {
            model.symlink_paths.guard().push_back((
                from.trim().to_string(),
                Some(to.trim().to_string())
            ));
        }

        let private_paths = model.private_paths.widget();
        let shared_paths = model.shared_paths.widget();
        let symlink_paths = model.symlink_paths.widget();

        let private_path_entry = &model.private_path_entry;

        let shared_path_from_entry = &model.shared_path_from_entry;
        let shared_path_to_entry = &model.shared_path_to_entry;
        let read_only_switch = &model.read_only_switch;

        let symlink_path_from_entry = &model.symlink_path_from_entry;
        let symlink_path_to_entry = &model.symlink_path_to_entry;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            SandboxPageMsg::AddPrivate => {
                if let Ok(mut config) = Config::get() {
                    let path = self.private_path_entry.text().trim().to_string();

                    if !path.is_empty() {
                        self.private_path_entry.set_text("");

                        config.sandbox.private.push(path.clone());

                        Config::update(config);

                        self.private_paths.guard().push_back((path, None));
                    }
                }
            }

            SandboxPageMsg::RemovePrivate(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.private_paths.guard().get(index.current_index()) {
                        config.sandbox.private.retain(|item| item != &var.from);

                        Config::update(config);
                    }

                    self.private_paths.guard().remove(index.current_index());
                }
            },

            SandboxPageMsg::AddShared => {
                if let Ok(mut config) = Config::get() {
                    let from = self.shared_path_from_entry.text().trim().to_string();
                    let to = self.shared_path_to_entry.text().trim().to_string();

                    let read_only = self.read_only_switch.state();

                    if !from.is_empty() && !to.is_empty() {
                        self.shared_path_from_entry.set_text("");
                        self.shared_path_to_entry.set_text("");

                        if read_only {
                            config.sandbox.mounts.read_only.insert(from.clone(), to.clone());
                        } else {
                            config.sandbox.mounts.bind.insert(from.clone(), to.clone());
                        }

                        Config::update(config);

                        self.shared_paths.guard().push_back((
                            from,
                            Some(if read_only {
                                format!("[read-only] {}", to)
                            } else {
                                to
                            })
                        ));
                    }
                }
            }

            SandboxPageMsg::RemoveShared(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.shared_paths.guard().get(index.current_index()) {
                        config.sandbox.mounts.read_only.remove(&var.from);
                        config.sandbox.mounts.bind.remove(&var.from);

                        Config::update(config);
                    }

                    self.shared_paths.guard().remove(index.current_index());
                }
            },

            SandboxPageMsg::AddSymlink => {
                if let Ok(mut config) = Config::get() {
                    let from = self.symlink_path_from_entry.text().trim().to_string();
                    let to = self.symlink_path_to_entry.text().trim().to_string();

                    if !from.is_empty() && !to.is_empty() {
                        self.symlink_path_from_entry.set_text("");
                        self.symlink_path_to_entry.set_text("");

                        config.sandbox.mounts.symlinks.insert(from.clone(), to.clone());

                        Config::update(config);

                        self.symlink_paths.guard().push_back((from, Some(to)));
                    }
                }
            }

            SandboxPageMsg::RemoveSymlink(index) => {
                if let Ok(mut config) = Config::get() {
                    if let Some(var) = self.symlink_paths.guard().get(index.current_index()) {
                        config.sandbox.mounts.symlinks.remove(&var.from);

                        Config::update(config);
                    }

                    self.symlink_paths.guard().remove(index.current_index());
                }
            }
        }
    }
}
