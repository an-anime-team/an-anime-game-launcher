use relm4::prelude::*;

use adw::prelude::*;

pub struct ComponentsList {
    _download_folder: String
}

#[derive(Debug)]
pub enum AppMsg {
    Install,
    Remove
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentsList {
    type Init = super::ComponentsListPattern;
    type Input = AppMsg;
    type Output = ();

    view! {
        group = adw::PreferencesGroup {}
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ComponentsList {
            _download_folder: init.download_folder
        };

        let widgets = view_output!();

        for group in init.groups {
            let group = super::ComponentGroup::builder()
                .launch(group)
                .detach();

            widgets.group.add(group.widget());
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called about dialog event: {:?}", msg);

        // todo
    }
}
