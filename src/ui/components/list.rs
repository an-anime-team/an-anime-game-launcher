use relm4::prelude::*;
use adw::prelude::*;

use super::ComponentGroupMsg;

#[derive(Debug, Clone)]
pub struct ComponentsListInit<T> {
    pub pattern: super::ComponentsListPattern,
    pub on_downloaded: Option<T>,
    pub on_deleted: Option<T>
}

pub struct ComponentsList<T> {
    pub show_recommended_only: bool,
    pub init: ComponentsListInit<T>,

    pub groups: Vec<AsyncController<super::ComponentGroup>>
}

#[derive(Debug)]
pub enum ComponentsListMsg {
    ShowRecommendedOnly(bool),
    CallOnDownloaded,
    CallOnDeleted
}

#[relm4::component(async, pub)]
impl<T: std::fmt::Debug + Clone + 'static> SimpleAsyncComponent for ComponentsList<T> {
    type Init = ComponentsListInit<T>;
    type Input = ComponentsListMsg;
    type Output = T;

    view! {
        group = adw::PreferencesGroup {}
    }

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
        match msg {
            ComponentsListMsg::ShowRecommendedOnly(state) => {
                self.show_recommended_only = state;

                // todo
                for group in &self.groups {
                    group.sender().send(ComponentGroupMsg::ShowRecommendedOnly(state)).unwrap();
                }
            }

            #[allow(unused_must_use)]
            ComponentsListMsg::CallOnDownloaded => if let Some(on_downloaded) = &self.init.on_downloaded {
                sender.output(on_downloaded.to_owned());
            }

            #[allow(unused_must_use)]
            ComponentsListMsg::CallOnDeleted => if let Some(on_deleted) = &self.init.on_deleted {
                sender.output(on_deleted.to_owned());
            }
        }
    }
}
