use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::i18n::tr;

use super::welcome::*;

static mut MAIN_WINDOW: Option<adw::Window> = None;

pub struct FirstRunApp {
    welcome: AsyncController<WelcomeApp>,

    toast_overlay: adw::ToastOverlay,
    carousel: adw::Carousel
}

#[derive(Debug, Clone)]
pub enum FirstRunAppMsg {
    ScrollToTosWarning,

    Toast {
        title: String,
        description: Option<String>
    }
}

#[relm4::component(pub)]
impl SimpleComponent for FirstRunApp {
    type Init = ();
    type Input = FirstRunAppMsg;
    type Output = ();

    view! {
        window = adw::Window {
            set_title: Some("Welcome"), // TODO: update this based on currently open page
            set_default_size: (780, 560),

            #[local_ref]
            toast_overlay -> adw::ToastOverlay {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        add_css_class: "flat"
                    },

                    #[local_ref]
                    carousel -> adw::Carousel {
                        set_allow_mouse_drag: false,

                        append = model.welcome.widget(),
                    },

                    adw::CarouselIndicatorDots {
                        set_carousel: Some(&carousel),
                        set_height_request: 32
                    }
                }
            }
        }
    }

    fn init(
        _parent: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing first run window");

        let toast_overlay = adw::ToastOverlay::new();
        let carousel = adw::Carousel::new();

        let model = Self {
            welcome: WelcomeApp::builder()
                .launch(())
                .detach(),

            toast_overlay,
            carousel
        };

        let toast_overlay = &model.toast_overlay;
        let carousel = &model.carousel;

        let widgets = view_output!();

        unsafe {
            MAIN_WINDOW = Some(widgets.window.clone());
        }

        ComponentParts { model, widgets } // will return soon
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        tracing::debug!("Called first run window event: {:?}", msg);

        match msg {
            FirstRunAppMsg::ScrollToTosWarning => {
                self.carousel.scroll_to(self.welcome.widget(), true);
            }

            FirstRunAppMsg::Toast { title, description } => unsafe {
                let toast = adw::Toast::new(&title);

                toast.set_timeout(5);

                if let Some(description) = description {
                    toast.set_button_label(Some(&tr("details")));

                    let dialog = adw::MessageDialog::new(MAIN_WINDOW.as_ref(), Some(&title), Some(&description));

                    dialog.add_response("close", &tr("close"));
                    dialog.add_response("save", &tr("save"));

                    dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);

                    #[allow(unused_must_use)]
                    dialog.connect_response(Some("save"), |_, _| {
                        let result = std::process::Command::new("xdg-open")
                            .arg(crate::DEBUG_FILE.as_os_str())
                            .output();

                        if let Err(err) = result {
                            tracing::error!("Failed to open debug file: {}", err);
                        }
                    });

                    toast.connect_button_clicked(move |_| {
                        dialog.show();
                    });
                }

                self.toast_overlay.add_toast(&toast);
            }
        }
    }
}
