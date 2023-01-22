use relm4::prelude::*;

use adw::prelude::*;

use std::path::PathBuf;

pub struct ComponentGroup {
    pub title: String,
    pub show_recommended_only: bool,

    pub versions: Vec<Controller<super::ComponentVersion>>
}

#[derive(Debug)]
pub enum AppMsg {
    ShowRecommendedOnly(bool)
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentGroup {
    type Init = (super::ComponentsListGroup, PathBuf);
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
            title: init.0.title,
            show_recommended_only: true,

            versions: init.0.versions
                .into_iter()
                .map(|version| {
                    super::ComponentVersion::builder()
                        .launch((version, init.1.clone()))
                        .detach()
                })
                .collect()
        };

        let widgets = view_output!();

        for version in &model.versions {
            widgets.group.add_row(version.widget());
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called component group [{}] event: {:?}", self.title, msg);

        match msg {
            AppMsg::ShowRecommendedOnly(state) => {
                self.show_recommended_only = state;

                // todo
                for version in &self.versions {
                    version.sender().send(super::version::AppMsg::ShowRecommendedOnly(state)).unwrap();
                }
            }
        }
    }
}
