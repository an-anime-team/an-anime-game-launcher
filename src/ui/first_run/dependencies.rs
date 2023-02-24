use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use anime_launcher_sdk::is_available;

use std::process::{Command, Stdio};

use super::main::FirstRunAppMsg;

pub struct DependenciesApp {
    show_arch: bool,
    show_debian: bool,
    show_fedora: bool
}

#[derive(Debug, Clone)]
pub enum DependenciesAppMsg {
    Continue,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DependenciesApp {
    type Init = ();
    type Input = DependenciesAppMsg;
    type Output = FirstRunAppMsg;

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: "You're missing some dependencies!",
                    add_css_class: "title-1"
                },

                gtk::Label {
                    set_label: "You must install some packages to your system before continue installation process",
    
                    set_justify: gtk::Justification::Center,
                    set_wrap: true,
                    set_margin_top: 32
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 16,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 16,

                        #[watch]
                        set_visible: model.show_arch,

                        gtk::Label {
                            set_label: "Arch (pacman)"
                        },

                        gtk::Entry {
                            set_text: "sudo pacman -Syu git xdelta3",
                            set_editable: false
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 16,

                        #[watch]
                        set_visible: model.show_debian,

                        gtk::Label {
                            set_label: "Debian / Ubuntu (apt)"
                        },

                        gtk::Entry {
                            set_text: "sudo apt install git xdelta3",
                            set_editable: false
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 16,

                        #[watch]
                        set_visible: model.show_fedora,

                        gtk::Label {
                            set_label: "Fedora (dnf)"
                        },

                        gtk::Entry {
                            set_text: "sudo dnf install git xdelta",
                            set_editable: false
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,
    
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 8,

                    gtk::Button {
                        set_label: "Check",
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => DependenciesAppMsg::Continue
                    },

                    gtk::Button {
                        set_label: "Exit",
                        add_css_class: "pill",

                        connect_clicked => DependenciesAppMsg::Exit
                    }
                }
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut model = Self {
            show_arch: false,
            show_debian: false,
            show_fedora: false
        };

        // Decide which packaging format used in system
        match Command::new("pacman").stdout(Stdio::null()).spawn() {
            Ok(_) => model.show_arch = true,

            Err(_) => match Command::new("apt").stdout(Stdio::null()).spawn() {
                Ok(_) => model.show_debian = true,

                Err(_) => match Command::new("dnf").stdout(Stdio::null()).spawn() {
                    Ok(_) => model.show_fedora = true,

                    Err(_) => {
                        model.show_arch = true;
                        model.show_debian = true;
                        model.show_fedora = true;
                    }
                }
            }
        }

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            #[allow(unused_must_use)]
            DependenciesAppMsg::Continue => {
                let packages = ["git", "xdelta3"];

                for package in packages {
                    if !is_available(package) {
                        sender.output(Self::Output::Toast {
                            title: format!("Package is not available: {package}"),
                            description: None
                        });

                        return;
                    }
                }

                sender.output(Self::Output::ScrollToDefaultPaths);
            }

            DependenciesAppMsg::Exit => {
                // TODO: relm4 has some function for it
                std::process::exit(0);
            }
        }
    }
}
