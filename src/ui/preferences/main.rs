use relm4::prelude::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::i18n::tr;

pub struct App;

#[derive(Debug)]
pub enum AppMsg {
    Test
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = gtk::Window;
    type Input = AppMsg;
    type Output = ();

    view! {
        preferences_window = adw::PreferencesWindow {
            set_title: Some(&tr("preferences")),
            set_default_size: (700, 560),
            set_hide_on_close: true,
            set_modal: true,

            #[template]
            add = &super::general::General,

            #[template]
            add = &super::enhancements::Enhancements,
            
            connect_close_request => |_| {
                anime_launcher_sdk::config::flush().unwrap(); // FIXME

                gtk::Inhibit::default()
            }
        }
    }

    fn init(
        parent: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App;
        let widgets = view_output!();

        widgets.preferences_window.set_transient_for(Some(&parent));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Test => {
                println!("sus");
            }
        }
    }
}
