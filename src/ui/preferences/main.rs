use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::components::*;

use crate::ui::components::{self, *};
use crate::i18n::tr;

use crate::CONFIG;

pub struct App {
    wine_components: Controller<ComponentsList>,
    dxvk_components: Controller<ComponentsList>
}

#[derive(Debug)]
pub enum AppMsg {
    WineRecommendedOnly(bool),
    DxvkRecommendedOnly(bool)
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = gtk::Window;
    type Input = AppMsg;
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
                wine_recommended_only {
                    connect_state_notify[sender] => move |switch| {
                        sender.input(AppMsg::WineRecommendedOnly(switch.state()));
                    }
                },

                #[template_child]
                dxvk_versions {
                    add = model.dxvk_components.widget(),
                },

                #[template_child]
                dxvk_recommended_only {
                    connect_state_notify[sender] => move |switch| {
                        sender.input(AppMsg::DxvkRecommendedOnly(switch.state()));
                    }
                },
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing preferences window");

        let model = App {
            wine_components: ComponentsList::builder()
                .launch(ComponentsListPattern {
                    download_folder: CONFIG.game.wine.builds.clone(),
                    groups: wine::get_groups().into_iter().map(|group| group.into()).collect()
                })
                .detach(),

            dxvk_components: ComponentsList::builder()
                .launch(ComponentsListPattern {
                    download_folder: CONFIG.game.dxvk.builds.clone(),
                    groups: dxvk::get_groups().into_iter().map(|group| group.into()).collect()
                })
                .detach(),
        };

        let widgets = view_output!();

        widgets.preferences_window.set_transient_for(Some(&parent));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called preferences window event: {:?}", msg);

        match msg {
            AppMsg::WineRecommendedOnly(state) => {
                // todo
                self.wine_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }

            AppMsg::DxvkRecommendedOnly(state) => {
                // todo
                self.dxvk_components.sender().send(components::list::AppMsg::ShowRecommendedOnly(state)).unwrap();
            }
        }
    }
}
