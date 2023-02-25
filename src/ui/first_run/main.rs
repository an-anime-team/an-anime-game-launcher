use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use crate::i18n::tr;

use super::welcome::*;
use super::tos_warning::*;
use super::dependencies::*;
use super::default_paths::*;
use super::select_voiceovers::*;
use super::download_components::*;
use super::finish::*;

pub static mut MAIN_WINDOW: Option<adw::Window> = None;

// TODO: add special page for launcher style selection

pub struct FirstRunApp {
    welcome: AsyncController<WelcomeApp>,
    tos_warning: AsyncController<TosWarningApp>,
    dependencies: AsyncController<DependenciesApp>,
    default_paths: AsyncController<DefaultPathsApp>,
    select_voiceovers: AsyncController<SelectVoiceoversApp>,
    download_components: AsyncController<DownloadComponentsApp>,
    finish: AsyncController<FinishApp>,

    toast_overlay: adw::ToastOverlay,
    carousel: adw::Carousel,

    title: String
}

#[derive(Debug, Clone)]
pub enum FirstRunAppMsg {
    ScrollToTosWarning,
    ScrollToDependencies,
    ScrollToDefaultPaths,
    ScrollToSelectVoiceovers,
    ScrollToDownloadComponents,
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
                        append = model.select_voiceovers.widget(),
                        append = model.download_components.widget(),
                        append = model.finish.widget(),
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

            select_voiceovers: SelectVoiceoversApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            download_components: DownloadComponentsApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            finish: FinishApp::builder()
                .launch(())
                .forward(sender.input_sender(), std::convert::identity),

            toast_overlay,
            carousel,

            title: tr("welcome")
        };

        let toast_overlay = &model.toast_overlay;
        let carousel = &model.carousel;

        let widgets = view_output!();

        unsafe {
            MAIN_WINDOW = Some(widgets.window.clone());
        }

        tracing::info!("First run window initialized");

        ComponentParts { model, widgets } // will return soon
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        tracing::debug!("Called first run window event: {:?}", msg);

        match msg {
            FirstRunAppMsg::ScrollToTosWarning => {
                self.title = tr("tos-violation-warning");

                self.carousel.scroll_to(self.tos_warning.widget(), true);
            }

            FirstRunAppMsg::ScrollToDependencies => {
                self.title = tr("dependencies");

                self.carousel.scroll_to(self.dependencies.widget(), true);
            }

            FirstRunAppMsg::ScrollToDefaultPaths => {
                self.title = tr("default-paths");

                self.carousel.scroll_to(self.default_paths.widget(), true);
            }

            FirstRunAppMsg::ScrollToSelectVoiceovers => {
                self.title = tr("select-voice-packages");

                self.carousel.scroll_to(self.select_voiceovers.widget(), true);
            }

            FirstRunAppMsg::ScrollToDownloadComponents => {
                self.title = tr("download-components");

                self.carousel.scroll_to(self.download_components.widget(), true);
            }

            FirstRunAppMsg::ScrollToFinish => {
                self.title = tr("finish");

                self.carousel.scroll_to(self.finish.widget(), true);
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
