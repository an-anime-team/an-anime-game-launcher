use relm4::prelude::*;
use relm4::component::*;

use adw::prelude::*;

#[derive(Debug, Clone)]
pub struct ComponentsListInit {
    pub pattern: super::ComponentsListPattern,
    pub on_downloaded: Option<crate::ui::preferences::main::AppMsg>,
    pub on_deleted: Option<crate::ui::preferences::main::AppMsg>
}

pub struct ComponentsList {
    pub show_recommended_only: bool,
    pub init: ComponentsListInit,

    pub groups: Vec<AsyncController<super::ComponentGroup>>
}

#[derive(Debug)]
pub enum AppMsg {
    ShowRecommendedOnly(bool),
    CallOnDownloaded,
    CallOnDeleted
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for ComponentsList {
    type Init = ComponentsListInit;
    type Input = AppMsg;
    type Output = crate::ui::preferences::main::AppMsg;

    view! {
        group = adw::PreferencesGroup {}
    }

    #[allow(clippy::redundant_clone)]
    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let init_copy = init.clone();

        let model = ComponentsList {
            show_recommended_only: true,
            init: init_copy,

            groups: init.pattern.groups
                .into_iter()
                .map(|group| {
                    super::ComponentGroup::builder()
                        .launch((group, init.pattern.download_folder.clone()))
                        .forward(sender.input_sender(), std::convert::identity)
                })
                .collect()
        };

        let widgets = view_output!();

        for group in &model.groups {
            widgets.group.add(group.widget());
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        tracing::debug!("Called components list event: {:?}", msg);

        match msg {
            AppMsg::ShowRecommendedOnly(state) => {
                self.show_recommended_only = state;

                // todo
                for group in &self.groups {
                    group.sender().send(super::group::AppMsg::ShowRecommendedOnly(state)).unwrap();
                }
            }

            #[allow(unused_must_use)]
            AppMsg::CallOnDownloaded => if let Some(on_downloaded) = self.init.on_downloaded {
                sender.output(on_downloaded);
            }

            #[allow(unused_must_use)]
            AppMsg::CallOnDeleted => if let Some(on_deleted) = self.init.on_deleted {
                sender.output(on_deleted);
            }
        }
    }
}
