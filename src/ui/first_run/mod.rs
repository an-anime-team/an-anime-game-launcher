use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

use gtk::glib;
use gtk::glib::clone;

use std::rc::Rc;
use std::cell::Cell;
use std::process::Command;

use anime_game_core::prelude::*;

mod welcome;
mod dependencies;
mod tos_warning;
mod default_paths;
mod voice_packages;
mod download_components;
mod finish;

use crate::ui::*;
use crate::ui::traits::prelude::*;
use crate::ui::components::progress_bar::*;

use crate::lib;
use crate::lib::wine_prefix::WinePrefix;
use crate::lib::config;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::default` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone)]
pub struct AppWidgets {
    pub window: adw::ApplicationWindow,
    pub toast_overlay: adw::ToastOverlay,
    pub carousel: adw::Carousel,

    pub welcome: welcome::Page,
    pub dependencies: dependencies::Page,
    pub tos_warning: tos_warning::Page,
    pub default_paths: default_paths::Page,
    pub voice_packages: voice_packages::Page,
    pub download_components: download_components::Page,
    pub finish: finish::Page
}

impl AppWidgets {
    pub fn try_get() -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run.ui");

        let result = Self {
            window: get_object(&builder, "window")?,
            toast_overlay: get_object(&builder, "toast_overlay")?,
            carousel: get_object(&builder, "carousel")?,

            welcome: welcome::Page::new()?,
            dependencies: dependencies::Page::new()?,
            tos_warning: tos_warning::Page::new()?,
            default_paths: default_paths::Page::new(get_object(&builder, "window")?)?,
            voice_packages: voice_packages::Page::new()?,
            download_components: download_components::Page::new()?,
            finish: finish::Page::new()?
        };

        // Add pages to carousel
        result.carousel.append(&result.welcome.page);
        result.carousel.append(&result.dependencies.page);
        result.carousel.append(&result.tos_warning.page);
        result.carousel.append(&result.default_paths.page);
        result.carousel.append(&result.voice_packages.page);
        result.carousel.append(&result.download_components.page);
        result.carousel.append(&result.finish.page);

        // Set devel style to ApplicationWindow if it's debug mode
        if crate::APP_DEBUG {
            result.window.add_css_class("devel");
        }

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
/// 
/// Has to implement glib::Downgrade` trait
#[derive(Debug, glib::Downgrade)]
pub enum Actions {
    WelcomeContinue,
    WelcomeAdvanced,
    DependenciesContinue,
    TosWarningContinue,
    DefaultPathsContinue,
    VoicePackagesContinue,
    DownloadComponents,
    DownloadComponentsContinue,
    Restart,
    Exit,
    Toast(Rc<(String, String)>)
}

impl Actions {
    pub fn into_fn<T: gtk::glib::IsA<gtk::Widget>>(&self, app: &App) -> Box<dyn Fn(&T)> {
        Box::new(clone!(@weak self as action, @strong app => move |_| {
            app.update(action).unwrap();
        }))
    }
}

/// The main application structure
/// 
/// `Default` macro automatically calls `AppWidgets::default`, i.e. loads UI file and reference its widgets
/// 
/// `Rc<Cell<Values>>` means this:
/// - `Rc` addeds ability to reference the same value from various clones of the structure.
///   This will guarantee us that inner `Cell<Values>` is the same for all the `App::clone()` values
/// - `Cell` addeds inner mutability to its value, so we can mutate it even without mutable reference.
/// 
/// So we have a shared reference to some value that can be changed without mutable reference.
/// That's what we need and what we use in `App::update` method
#[derive(Clone)]
pub struct App {
    widgets: AppWidgets,
    actions: Rc<Cell<Option<glib::Sender<Actions>>>>,
    advanced: Rc<Cell<bool>>
}

impl App {
    /// Create new application
    pub fn new(app: &gtk::Application) -> Result<Self, String> {
        // Get default widgets from ui file and add events to them
        let result = Self {
            widgets: AppWidgets::try_get()?,
            actions: Default::default(),
            advanced: Default::default()
        }.init_events().init_actions();

        // Bind app to the window
        result.widgets.window.set_application(Some(app));

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        self.widgets.welcome.continue_button.connect_clicked(Actions::WelcomeContinue.into_fn(&self));
        self.widgets.tos_warning.continue_button.connect_clicked(Actions::TosWarningContinue.into_fn(&self));
        self.widgets.default_paths.continue_button.connect_clicked(Actions::DefaultPathsContinue.into_fn(&self));
        self.widgets.dependencies.check_button.connect_clicked(Actions::DependenciesContinue.into_fn(&self));
        self.widgets.voice_packages.continue_button.connect_clicked(Actions::VoicePackagesContinue.into_fn(&self));

        self.widgets.welcome.advanced_button.connect_clicked(Actions::WelcomeAdvanced.into_fn(&self));
        self.widgets.download_components.download_button.connect_clicked(Actions::DownloadComponents.into_fn(&self));

        self.widgets.dependencies.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.tos_warning.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.default_paths.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.voice_packages.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.download_components.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.finish.exit_button.connect_clicked(Actions::Exit.into_fn(&self));

        self.widgets.finish.restart_button.connect_clicked(Actions::Restart.into_fn(&self));

        self
    }

    /// Add actions processors
    /// 
    /// Changes will happen in the main thread so you can call `update` method from separate thread
    pub fn init_actions(self) -> Self {
        let (sender, receiver) = glib::MainContext::channel::<Actions>(glib::PRIORITY_DEFAULT);

        let this = self.clone();

        receiver.attach(None, move |action| {
            // Some debug output
            println!("[update] action: {:?}", &action);

            match action {
                Actions::WelcomeContinue => {
                    this.widgets.carousel.scroll_to({
                        if lib::is_available("git") && lib::is_available("xdelta3") {
                            &this.widgets.tos_warning.page
                        } else {
                            &this.widgets.dependencies.page
                        }
                    }, true);
                }

                Actions::WelcomeAdvanced => {
                    this.advanced.set(true);

                    this.update(Actions::WelcomeContinue).unwrap();
                }

                Actions::DependenciesContinue => {
                    let mut installed = true;

                    for package in ["git", "xdelta3"] {
                        if !lib::is_available(package) {
                            installed = false;

                            this.toast(format!("Package {package} is not installed"), "");

                            break;
                        }
                    }

                    if installed {
                        this.widgets.carousel.scroll_to(&this.widgets.tos_warning.page, true);
                    }
                }

                Actions::TosWarningContinue => {
                    this.widgets.carousel.scroll_to({
                        if this.advanced.get() {
                            &this.widgets.default_paths.page
                        } else {
                            &this.widgets.voice_packages.page
                        }
                    }, true);
                }

                Actions::DefaultPathsContinue => {
                    config::update_raw(this.widgets.default_paths.update_config(config::get().unwrap())).unwrap();

                    this.widgets.carousel.scroll_to(&this.widgets.voice_packages.page, true);
                }

                Actions::VoicePackagesContinue => {
                    config::update_raw(this.widgets.voice_packages.update_config(config::get().unwrap())).unwrap();

                    this.widgets.carousel.scroll_to(&this.widgets.download_components.page, true);
                }

                Actions::DownloadComponents => {
                    this.widgets.download_components.wine_version.set_sensitive(false);
                    this.widgets.download_components.dxvk_version.set_sensitive(false);

                    this.widgets.download_components.progress_bar.show();

                    let (sender_wine, receiver_wine) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);
                    let (sender_dxvk, receiver_dxvk) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);

                    let progress_bar = this.widgets.download_components.progress_bar.clone();

                    let wine_version = this.widgets.download_components.get_wine_version().clone();
                    let dxvk_version = this.widgets.download_components.get_dxvk_version().clone();

                    let wine_version_copy = wine_version.clone();
                    let this_copy = this.clone();

                    // Prepare wine downloader
                    std::thread::spawn(move || {
                        let config = config::get().unwrap();

                        match Installer::new(&wine_version_copy.uri) {
                            Ok(mut installer) => {
                                if let Some(temp_folder) = config.launcher.temp {
                                    installer.temp_folder = temp_folder;
                                }

                                installer.downloader
                                    .set_downloading_speed(config.launcher.speed_limit)
                                    .expect("Failed to set downloading speed limit");

                                // Download wine
                                #[allow(unused_must_use)]
                                installer.install(&config.game.wine.builds, move |state| {
                                    sender_wine.send(state);
                                });
                            },
                            Err(err) => {
                                this_copy.update(Actions::Toast(Rc::new((
                                    String::from("Failed to init wine downloader"), err.to_string()
                                )))).unwrap();
                            }
                        }
                    });

                    // Display wine downloading progress
                    let progress_bar_copy = progress_bar.clone();
                    let dxvk_version_copy = dxvk_version.clone();

                    let this_copy = this.clone();

                    receiver_wine.attach(None, move |state| {
                        match progress_bar_copy.update_from_state(state) {
                            ProgressUpdateResult::Updated => (),

                            ProgressUpdateResult::Error(msg, err) => {
                                this_copy.toast(msg, err);
                            },

                            ProgressUpdateResult::Finished => {
                                let mut config = config::get().unwrap();
                                let prefix = WinePrefix::new(&config.game.wine.prefix);

                                // Update wine config
                                config.game.wine.selected = Some(wine_version.name.clone());

                                config::update_raw(config.clone()).unwrap();

                                // Create wine prefix
                                let this = this_copy.clone();
                                let wine_version = wine_version.clone();
                                let dxvk_version = dxvk_version_copy.clone();
                                let sender_dxvk = sender_dxvk.clone();

                                std::thread::spawn(move || {
                                    match prefix.update(&config.game.wine.builds, wine_version.clone()) {
                                        Ok(output) => {
                                            println!("Wine prefix created:\n\n{}", String::from_utf8_lossy(&output.stdout));
    
                                            // Prepare DXVK downloader
                                            match Installer::new(&dxvk_version.uri) {
                                                Ok(mut installer) => {
                                                    if let Some(temp_folder) = config.launcher.temp {
                                                        installer.temp_folder = temp_folder;
                                                    }

                                                    installer.downloader
                                                        .set_downloading_speed(config.launcher.speed_limit)
                                                        .expect("Failed to set downloading speed limit");
    
                                                    // Download DXVK
                                                    #[allow(unused_must_use)]
                                                    installer.install(&config.game.dxvk.builds, move |state| {
                                                        sender_dxvk.send(state);
                                                    });
                                                },
                                                Err(err) => {
                                                    this.update(Actions::Toast(Rc::new((
                                                        String::from("Failed to init DXVK downloader"), err.to_string()
                                                    )))).unwrap();
                                                }
                                            }
                                        },
                                        Err(err) => {
                                            this.update(Actions::Toast(Rc::new((
                                                String::from("Failed to create wine prefix"), err.to_string()
                                            )))).unwrap();
                                        }
                                    }
                                });

                                return glib::Continue(false);
                            }
                        }

                        glib::Continue(true)
                    });

                    // Display DXVK downloading progress
                    let this = this.clone();

                    receiver_dxvk.attach(None, move |state| {
                        match progress_bar.update_from_state(state) {
                            ProgressUpdateResult::Updated => (),

                            ProgressUpdateResult::Error(msg, err) => {
                                this.toast(msg, err);
                            },

                            ProgressUpdateResult::Finished => {
                                let config = config::get().unwrap();

                                // Apply DXVK
                                let this = this.clone();
                                let dxvk_version = dxvk_version.clone();

                                std::thread::spawn(move || {
                                    match dxvk_version.apply(&config.game.dxvk.builds, &config.game.wine.prefix) {
                                        Ok(output) => {
                                            println!("Applied DXVK:\n\n{}", String::from_utf8_lossy(&output.stdout));

                                            // Remove .first-run file
                                            let launcher_dir = crate::lib::consts::launcher_dir().unwrap();
    
                                            std::fs::remove_file(format!("{}/.first-run", launcher_dir)).unwrap();
    
                                            // Show next page
                                            this.update(Actions::DownloadComponentsContinue).unwrap();
                                        },
                                        Err(err) => {
                                            this.update(Actions::Toast(Rc::new((
                                                String::from("Failed to apply DXVK"), err.to_string()
                                            )))).unwrap();
                                        }
                                    }
                                });

                                return glib::Continue(false);
                            }
                        }

                        glib::Continue(true)
                    });
                }

                Actions::DownloadComponentsContinue => {
                    this.widgets.carousel.scroll_to(&this.widgets.finish.page, true);
                }

                Actions::Restart => {
                    Command::new(std::env::current_exe().unwrap()).spawn().unwrap();

                    this.widgets.window.close();
                }

                Actions::Exit => {
                    this.widgets.window.close();
                }

                Actions::Toast(toast) => {
                    let (msg, err) = (toast.0.clone(), toast.1.clone());

                    this.toast(msg, err);
                }
            }

            glib::Continue(true)
        });

        self.actions.set(Some(sender));

        self
    }

    /// Update widgets state by calling some action
    pub fn update(&self, action: Actions) -> Result<(), std::sync::mpsc::SendError<Actions>> {
        let actions = self.actions.take();
        
        let result = match &actions {
            Some(sender) => Ok(sender.send(action)?),
            None => Ok(())
        };

        self.actions.set(actions);

        result
    }

    /// Show application window
    pub fn show(&self) {
        self.widgets.window.show();
    }
}

impl Toast for App {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay) {
        (self.widgets.window.clone(), self.widgets.toast_overlay.clone())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
