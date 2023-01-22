use relm4::prelude::*;

use adw::prelude::*;

pub struct ComponentsList {
    pub show_recommended_only: bool,

    pub groups: Vec<Controller<super::ComponentGroup>>
}

#[derive(Debug)]
pub enum AppMsg {
    ShowRecommendedOnly(bool)
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
            show_recommended_only: true,

            groups: init.groups
                .into_iter()
                .map(|group| {
                    super::ComponentGroup::builder()
                        .launch((group, init.download_folder.clone()))
                        .detach()
                })
                .collect()
        };

        let widgets = view_output!();

        for group in &model.groups {
            widgets.group.add(group.widget());
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called components list event: {:?}", msg);

        match msg {
            AppMsg::ShowRecommendedOnly(state) => {
                self.show_recommended_only = state;

                // todo
                for group in &self.groups {
                    group.sender().send(super::group::AppMsg::ShowRecommendedOnly(state)).unwrap();
                }
            }
        }
    }
}
