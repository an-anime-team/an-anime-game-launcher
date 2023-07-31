use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;

use crate::tr;

use super::first_run::default_paths::DefaultPathsApp;

pub struct MigrateInstallationApp {
    default_paths: AsyncController<DefaultPathsApp>,
}

#[relm4::component(pub)]
impl SimpleComponent for MigrateInstallationApp {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::Window {
            set_default_size: (780, 560),
            set_modal: true,
            set_hide_on_close: true,

            #[watch]
            set_title: Some(&tr!("migrate-installation")),

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
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing migration window");

        let model = Self {
            default_paths: DefaultPathsApp::builder()
                .launch(true)
                .detach()
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
