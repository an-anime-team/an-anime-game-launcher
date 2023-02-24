use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::i18n::tr;

use super::welcome::*;
use super::tos_warning::*;
use super::dependencies::*;
use super::default_paths::*;

pub static mut MAIN_WINDOW: Option<adw::Window> = None;

pub struct FirstRunApp {
    welcome: AsyncController<WelcomeApp>,
    tos_warning: AsyncController<TosWarningApp>,
    dependencies: AsyncController<DependenciesApp>,
    default_paths: AsyncController<DefaultPathsApp>,

    toast_overlay: adw::ToastOverlay,
    carousel: adw::Carousel,

    title: String
}

#[derive(Debug, Clone)]
pub enum FirstRunAppMsg {
    ScrollToTosWarning,
    ScrollToDependencies,
    ScrollToDefaultPaths,
    ScrollToDownloadComponents,
    ScrollToChooseVoiceovers,
    ScrollToFinish,

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
            set_default_size: (780, 560),

            #[watch]
            set_title: Some(&model.title),

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
                        append = model.tos_warning.widget(),
                        append = model.dependencies.widget(),
                        append = model.default_paths.widget(),
                    },

                    adw::CarouselIndicatorDots {
                        set_carousel: Some(carousel),
                        set_height_request: 32
                    }
                }
            }
        }
    }

    fn init(
        _parent: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing first run window");

        let toast_overlay = adw::ToastOverlay::new();
        let carousel = adw::Carousel::new();

        let model = Self {
            welcome: WelcomeApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            tos_warning: TosWarningApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            dependencies: DependenciesApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            default_paths: DefaultPathsApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            toast_overlay,
            carousel,

            title: String::from("Welcome")
        };

        let toast_overlay = &model.toast_overlay;
        let carousel = &model.carousel;

        let widgets = view_output!();

        unsafe {
            MAIN_WINDOW = Some(widgets.window.clone());
        }

        ComponentParts { model, widgets } // will return soon
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called first run window event: {:?}", msg);

        match msg {
            FirstRunAppMsg::ScrollToTosWarning => {
                self.title = String::from("ToS Warning");

                self.carousel.scroll_to(self.tos_warning.widget(), true);
            }

            FirstRunAppMsg::ScrollToDependencies => {
                self.title = String::from("Dependencies");

                self.carousel.scroll_to(self.dependencies.widget(), true);
            }

            FirstRunAppMsg::ScrollToDefaultPaths => {
                self.title = String::from("Default paths");

                self.carousel.scroll_to(self.default_paths.widget(), true);
            }

            FirstRunAppMsg::ScrollToDownloadComponents => {
                self.title = String::from("Download components");

                self.carousel.scroll_to(self.welcome.widget(), true);
            }

            FirstRunAppMsg::ScrollToChooseVoiceovers => {
                self.title = String::from("Select voiceovers");

                self.carousel.scroll_to(self.welcome.widget(), true);
            }

            FirstRunAppMsg::ScrollToFinish => {
                self.title = String::from("Finish");

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
