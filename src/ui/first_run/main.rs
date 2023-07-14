use relm4::prelude::*;
use relm4::component::*;

use gtk::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::components::loader::ComponentsLoader;

use crate::i18n::tr;
use crate::*;

use super::welcome::*;
use super::tos_warning::*;
use super::dependencies::*;
use super::default_paths::*;
use super::select_voiceovers::*;
use super::download_components::*;
use super::finish::*;

pub static mut MAIN_WINDOW: Option<adw::ApplicationWindow> = None;

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

    loading: Option<Option<String>>,
    title: String
}

#[derive(Debug, Clone)]
pub enum FirstRunAppMsg {
    SetLoadingStatus(Option<Option<String>>),

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
        window = adw::ApplicationWindow {
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

                    adw::StatusPage {
                        set_title: &tr("loading-data"),
                        set_icon_name: Some(APP_ID),
                        set_vexpand: true,

                        #[watch]
                        set_description: match &model.loading {
                            Some(Some(desc)) => Some(desc),
                            Some(None) | None => None
                        },

                        #[watch]
                        set_visible: model.loading.is_some()
                    },

                    #[local_ref]
                    carousel -> adw::Carousel {
                        #[watch]
                        set_visible: model.loading.is_none(),

                        set_allow_mouse_drag: false,
                        set_allow_long_swipes: false,
                        set_allow_scroll_wheel: false,

                        append = model.welcome.widget(),
                        append = model.tos_warning.widget(),
                        append = model.dependencies.widget(),
                        append = model.default_paths.widget(),
                        append = model.select_voiceovers.widget(),
                        append = model.download_components.widget(),
                        append = model.finish.widget(),
                    },

                    adw::CarouselIndicatorDots {
                        #[watch]
                        set_visible: model.loading.is_none(),

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
                .launch(false)
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

            loading: None,
            title: tr("welcome")
        };

        let toast_overlay = &model.toast_overlay;
        let carousel = &model.carousel;

        let widgets = view_output!();

        unsafe {
            MAIN_WINDOW = Some(widgets.window.clone());
        }

        crate::READY.store(true, Ordering::Relaxed);

        tracing::info!("First run window initialized. App is ready");

        ComponentParts { model, widgets } // will return soon
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        tracing::debug!("Called first run window event: {:?}", msg);

        match msg {
            FirstRunAppMsg::SetLoadingStatus(status) => {
                self.loading = status;
            }

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
                // Update components index
                sender.input(FirstRunAppMsg::SetLoadingStatus(Some(Some(tr("updating-components-index")))));

                let config = Config::get().unwrap_or_else(|_| CONFIG.clone());

                let components_sender = self.download_components.sender().clone();
                let components = ComponentsLoader::new(config.components.path);

                #[allow(unused_must_use)]
                std::thread::spawn(move || {
                    match components.is_sync(config.components.servers) {
                        Ok(Some(_)) => (),

                        Ok(None) => {
                            for host in &CONFIG.components.servers {
                                match components.sync(host) {
                                    Ok(_) => break,

                                    Err(err) => {
                                        tracing::error!("Failed to sync components index");

                                        sender.input(FirstRunAppMsg::Toast {
                                            title: tr("components-index-sync-failed"),
                                            description: Some(err.to_string())
                                        });
                                    }
                                }
                            }
                        }

                        Err(err) => {
                            tracing::error!("Failed to verify that components index synced");

                            sender.input(FirstRunAppMsg::Toast {
                                title: tr("components-index-verify-failed"),
                                description: Some(err.to_string())
                            });
                        }
                    }

                    // Update versions lists in download components page
                    components_sender.send(DownloadComponentsAppMsg::UpdateVersionsLists);

                    // Hide status page
                    sender.input(FirstRunAppMsg::SetLoadingStatus(None));
                });

                // Scroll to download components page
                // This will happen in background behind StatusPage
                self.title = tr("download-components");

                self.carousel.scroll_to(self.download_components.widget(), true);
            }

            FirstRunAppMsg::ScrollToFinish => {
                self.title = tr("finish");

                self.carousel.scroll_to(self.finish.widget(), true);
            }

            FirstRunAppMsg::Toast { title, description } => unsafe {
                let toast = adw::Toast::new(&title);

                toast.set_timeout(4);

                if let Some(description) = description {
                    toast.set_button_label(Some(&tr("details")));

                    let dialog = adw::MessageDialog::new(MAIN_WINDOW.as_ref(), Some(&title), Some(&description));

                    dialog.add_response("close", &tr("close"));
                    dialog.add_response("save", &tr("save"));

                    dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);

                    dialog.connect_response(Some("save"), |_, _| {
                        if let Err(err) = open::that(crate::DEBUG_FILE.as_os_str()) {
                            tracing::error!("Failed to open debug file: {err}");
                        }
                    });

                    toast.connect_button_clicked(move |_| {
                        dialog.present();
                    });
                }

                self.toast_overlay.add_toast(toast);
            }
        }
    }
}
