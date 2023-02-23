use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

use std::path::PathBuf;

pub struct ComponentGroup {
    pub title: String,
    pub show_recommended_only: bool,

    pub versions: Vec<AsyncController<super::ComponentVersion>>
}

#[derive(Debug)]
pub enum AppMsg {
    ShowRecommendedOnly(bool),
    CallOnDownloaded,
    CallOnDeleted
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for ComponentGroup {
    type Init = (super::ComponentsListGroup, PathBuf);
    type Input = AppMsg;
    type Output = super::list::AppMsg;

    view! {
        group = adw::ExpanderRow {
            set_title: &model.title
        }
    }

    #[allow(clippy::redundant_clone)]
    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = ComponentGroup {
            title: init.0.title,
            show_recommended_only: true,

            versions: init.0.versions
                .into_iter()
                .map(|version| {
                    super::ComponentVersion::builder()
                        .launch((version, init.1.clone()))
                        .forward(sender.input_sender(), std::convert::identity)
                })
                .collect()
        };

        let widgets = view_output!();

        for version in &model.versions {
            widgets.group.add_row(version.widget());
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            AppMsg::ShowRecommendedOnly(state) => {
                self.show_recommended_only = state;

                // todo
                for version in &self.versions {
                    version.sender().send(super::version::AppMsg::ShowRecommendedOnly(state)).unwrap();
                }
            }

            #[allow(unused_must_use)]
            AppMsg::CallOnDownloaded => {
                sender.output(super::list::AppMsg::CallOnDownloaded);
            }

            #[allow(unused_must_use)]
            AppMsg::CallOnDeleted => {
                sender.output(super::list::AppMsg::CallOnDeleted);
            }
        }
    }
}
