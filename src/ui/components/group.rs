use relm4::prelude::*;

use adw::prelude::*;

pub struct ComponentGroup {
    pub title: String
}

#[derive(Debug)]
pub enum AppMsg {
    Install,
    Remove
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentGroup {
    type Init = super::ComponentsListGroup;
    type Input = AppMsg;
    type Output = ();

    view! {
        group = adw::ExpanderRow {
            set_title: &model.title
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ComponentGroup {
            title: init.title
        };

        let widgets = view_output!();

        for version in init.versions {
            let version = super::ComponentVersion::builder()
                .launch(version)
                .detach();

            widgets.group.add_row(version.widget());
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called about dialog event: {:?}", msg);

        // todo
    }
}
