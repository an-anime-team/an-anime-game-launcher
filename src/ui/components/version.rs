use relm4::prelude::*;

use adw::prelude::*;

pub struct ComponentVersion {
    pub title: String
}

#[derive(Debug)]
pub enum AppMsg {
    Install,
    Remove
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentVersion {
    type Init = super::ComponentsListVersion;
    type Input = AppMsg;
    type Output = ();

    view! {
        row = adw::ActionRow {
            set_title: &model.title
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ComponentVersion {
            title: init.title
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called about dialog event: {:?}", msg);

        // todo
    }
}
