use relm4::prelude::*;

use adw::prelude::*;

pub struct ComponentVersion {
    pub title: String,
    pub recommended: bool,
    pub show_recommended_only: bool
}

#[derive(Debug)]
pub enum AppMsg {
    ShowRecommendedOnly(bool)
}

#[relm4::component(pub)]
impl SimpleComponent for ComponentVersion {
    type Init = super::ComponentsListVersion;
    type Input = AppMsg;
    type Output = ();

    view! {
        row = adw::ActionRow {
            set_title: &model.title,

            #[watch]
            set_visible: !model.show_recommended_only || model.recommended
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ComponentVersion {
            title: init.title,
            recommended: init.recommended,
            show_recommended_only: true
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called component version [{}] event: {:?}", self.title, msg);

        match msg {
            AppMsg::ShowRecommendedOnly(state) => self.show_recommended_only = state
        }
    }
}
