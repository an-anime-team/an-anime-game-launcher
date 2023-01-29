use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::anime_game_core::prelude::*;

use crate::i18n::*;
use crate::*;

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
                        set_text: &match GAME_DIFF.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(current) |
                                VersionDiff::Predownload { current, .. } |
                                VersionDiff::Diff { current, .. } |
                                VersionDiff::Outdated { current, .. } => current.to_string(),

                                VersionDiff::NotInstalled { .. } => tr("game-not-installed")
                            }

                            None => String::from("?")
                        },

                        add_css_class: match GAME_DIFF.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(_) => "success",
                                VersionDiff::Predownload { .. } => "accent",
                                VersionDiff::Diff { .. } => "warning",
                                VersionDiff::Outdated { .. } => "error",
                                VersionDiff::NotInstalled { .. } => ""
                            }

                            None => "success"
                        },

                        set_tooltip_text: Some(&match GAME_DIFF.as_ref() {
                            Some(diff) => match diff {
                                VersionDiff::Latest(_) => String::new(),
                                VersionDiff::Predownload { current, latest, .. } => tr_args("game-predownload-available", [
                                    ("old", current.to_string().into()),
                                    ("new", latest.to_string().into())
                                ]),
                                VersionDiff::Diff { current, latest, .. } => tr_args("game-update-available", [
                                    ("old", current.to_string().into()),
                                    ("new", latest.to_string().into())
                                ]),
                                VersionDiff::Outdated { latest, ..} => tr_args("game-outdated", [
                                    ("latest", latest.to_string().into())
                                ]),
                                VersionDiff::NotInstalled { .. } => String::new()
                            }

                            None => String::new()
                        })
                    }
                },

                adw::ActionRow {
                    set_title: &tr("patch-version"),

                    add_suffix = &gtk::Label {
                        set_text: &match PATCH.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => tr("patch-not-available"),
                                Patch::Outdated { current, .. } => tr_args("patch-outdated", [("current", current.to_string().into())]),
                                Patch::Preparation { .. } => tr("patch-preparation"),
                                Patch::Testing { version, .. } |
                                Patch::Available { version, .. } => version.to_string()
                            }

                            None => String::from("?")
                        },

                        add_css_class: match PATCH.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => "error",
                                Patch::Outdated { .. } |
                                Patch::Preparation { .. } |
                                Patch::Testing { .. } => "warning",
                                Patch::Available { .. } => unsafe {
                                    if let Ok(true) = PATCH.as_ref().unwrap_unchecked().is_applied(&CONFIG.game.path) {
                                        "success"
                                    } else {
                                        "warning"
                                    }
                                }
                            }

                            None => ""
                        },

                        set_tooltip_text: Some(&match PATCH.as_ref() {
                            Some(patch) => match patch {
                                Patch::NotAvailable => tr("patch-not-available-tooltip"),
                                Patch::Outdated { current, latest, .. } => tr_args("patch-outdated-tooltip", [
                                    ("current", current.to_string().into()),
                                    ("latest", latest.to_string().into())
                                ]),
                                Patch::Preparation { .. } => tr("patch-preparation-tooltip"),
                                Patch::Testing { .. } => tr("patch-testing-tooltip"),
                                Patch::Available { .. } => unsafe {
                                    if let Ok(true) = PATCH.as_ref().unwrap_unchecked().is_applied(&CONFIG.game.path) {
                                        String::new()
                                    } else {
                                        tr("patch-testing-tooltip")
                                    }
                                }
                            }

                            None => String::new()
                        })
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_title: &tr("wine-version"),

                adw::ComboRow {
                    set_title: &tr("selected-version")
                },

                adw::ActionRow {
                    set_title: &tr("recommended-only"),
                    set_subtitle: &tr("wine-recommended-description"),

                    #[name(wine_recommended_only)]
                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_state: true
                    }
                }
            },

            #[name(wine_versions)]
            add = &adw::PreferencesGroup {},

            add = &adw::PreferencesGroup {
                set_title: &tr("dxvk-version"),

                adw::ComboRow {
                    set_title: &tr("selected-version")
                },

                adw::ActionRow {
                    set_title: &tr("recommended-only"),
                    set_subtitle: &tr("dxvk-recommended-description"),

                    #[name(dxvk_recommended_only)]
                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        set_state: true
                    }
                }
            },

            #[name(dxvk_versions)]
            add = &adw::PreferencesGroup {},
        }
    }
}
