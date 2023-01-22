use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::ui::components::*;

use crate::i18n::tr;

pub struct App {
    wine_components: Controller<ComponentsList>,
    dxvk_components: Controller<ComponentsList>
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = gtk::Window;
    type Input = ();
    type Output = ();

    view! {
        preferences_window = adw::PreferencesWindow {
            set_title: Some(&tr("preferences")),
            set_default_size: (700, 560),
            set_hide_on_close: true,
            set_modal: true,

            #[template]
            add = &super::general::General {
                // Here technically it's AdwPreferencesGroup inside of AdwPreferencesGroup
                // but I have no idea how to do it other way
                // There're no graphical glitches so don't care

                #[template_child]
                wine_versions {
                    add = model.wine_components.widget(),
                },

                #[template_child]
                dxvk_versions {
                    add = model.dxvk_components.widget(),
                }
            },

            #[template]
            add = &super::enhancements::Enhancements,
            
            connect_close_request => |_| {
                anime_launcher_sdk::config::flush().unwrap(); // FIXME

                gtk::Inhibit::default()
            }
        }
    }

    fn init(
        parent: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            wine_components: ComponentsList::builder()
                .launch(ComponentsListPattern {
                    download_folder: String::from("/tmp"),
                    groups: vec![
                        ComponentsListGroup {
                            title: String::from("Test group 1"),
                            versions: vec![
                                ComponentsListVersion {
                                    title: String::from("Test version 1"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 2"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 3"),
                                    url: String::from("/")
                                }
                            ]
                        },
                        ComponentsListGroup {
                            title: String::from("Test group 2"),
                            versions: vec![
                                ComponentsListVersion {
                                    title: String::from("Test version 1"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 2"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 3"),
                                    url: String::from("/")
                                }
                            ]
                        }
                    ]
                })
                .detach(),

            dxvk_components: ComponentsList::builder()
                .launch(ComponentsListPattern {
                    download_folder: String::from("/tmp"),
                    groups: vec![
                        ComponentsListGroup {
                            title: String::from("Test group 1"),
                            versions: vec![
                                ComponentsListVersion {
                                    title: String::from("Test version 1"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 2"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 3"),
                                    url: String::from("/")
                                }
                            ]
                        },
                        ComponentsListGroup {
                            title: String::from("Test group 2"),
                            versions: vec![
                                ComponentsListVersion {
                                    title: String::from("Test version 1"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 2"),
                                    url: String::from("/")
                                },
                                ComponentsListVersion {
                                    title: String::from("Test version 3"),
                                    url: String::from("/")
                                }
                            ]
                        }
                    ]
                })
                .detach(),
        };

        let widgets = view_output!();

        widgets.preferences_window.set_transient_for(Some(&parent));

        ComponentParts { model, widgets }
    }
}
