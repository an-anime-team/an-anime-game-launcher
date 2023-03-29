use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::*;

use super::first_run::default_paths::DefaultPathsApp;

pub struct MigrateInstallationApp {
    default_paths: AsyncController<DefaultPathsApp>,
}

#[derive(Debug)]
pub enum MigrateInstallationAppMsg {
    Migrate
}

#[relm4::component(pub)]
impl SimpleComponent for MigrateInstallationApp {
    type Init = ();
    type Input = MigrateInstallationAppMsg;
    type Output = ();

    view! {
        adw::Window {
            set_default_size: (780, 560),
            set_modal: true,

            #[watch]
            set_title: Some("Migrate installation"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    add_css_class: "flat"
                },

                append = model.default_paths.widget(),
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing migration window");

        let model = Self {
            default_paths: DefaultPathsApp::builder()
                .launch(())
                .detach()
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            MigrateInstallationAppMsg::Migrate => {
                todo!()
            }
        }
    }
}
