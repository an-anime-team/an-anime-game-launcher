use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::config;

use crate::i18n::tr;
use crate::ui::main::is_ready;

lazy_static::lazy_static! {
    static ref CONFIG: config::Config = config::get().expect("Failed to load config");
}

#[relm4::widget_template(pub)]
impl WidgetTemplate for General {
    view! {
        adw::PreferencesPage {
            set_title: &tr("general"),
            set_icon_name: Some("applications-system-symbolic"),

            add = &adw::PreferencesGroup {
                set_title: &tr("general"),

                adw::ComboRow {
                    set_title: &tr("launcher-language"),
                    set_subtitle: &tr("launcher-language-description"),

                    // TODO: maybe simplify it by some way? e.g. specify such stuff in i18n mod

                    #[wrap(Some)]
                    set_model = &gtk::StringList::new(&[
                        "English",
                        "Русский"
                    ]),

                    set_selected: match CONFIG.launcher.language.as_str() {
                        "en-us" => 0,
                        "ru-ru" => 1,
                        _ => 0
                    },

                    connect_selected_notify => move |row| {
                        if is_ready() {
                            if let Ok(mut config) = config::get() {
                                config.launcher.language = String::from(*[
                                    "en-us",
                                    "ru-ru"
                                ].get(row.selected() as usize).unwrap_or(&"en-us"));
    
                                config::update(config);
                            }
                        }
                    }
                },

                adw::ExpanderRow {
                    set_title: &tr("game-voiceovers"),

                    add_row = &adw::ActionRow {
                        set_title: &tr("english"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("japanese"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("korean"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    },

                    add_row = &adw::ActionRow {
                        set_title: &tr("chinese"),

                        add_suffix = &gtk::Button {
                            add_css_class: "flat",
                            set_icon_name: "user-trash-symbolic",
                            set_valign: gtk::Align::Center
                        }
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_margin_top: 16,

                    gtk::Button {
                        set_label: &tr("repair-game")
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("status"),

                adw::ActionRow {
                    set_title: &tr("game-version"),

                    add_suffix = &gtk::Label {
                        set_text: "3.3.0",
                        add_css_class: "success"
                    }
                },

                adw::ActionRow {
                    set_title: &tr("patch-version"),

                    add_suffix = &gtk::Label {
                        set_text: "3.3.0",
                        add_css_class: "success"
                    }
                }
            }
        }
    }
}
