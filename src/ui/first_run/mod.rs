use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

use gtk::glib;
use gtk::glib::clone;

use std::rc::Rc;
use std::cell::Cell;
use std::process::{Command, Stdio};

use anime_game_core::prelude::*;

mod page_1;
mod page_2;
mod page_3;
mod page_4;
mod page_5;
mod page_6;

use crate::ui::*;
use crate::ui::components::progress_bar::*;

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
    pub carousel: adw::Carousel,

    // TODO: use names instead of numbers
    pub page_1: page_1::Page,
    pub page_2: page_2::Page,
    pub page_3: page_3::Page,
    pub page_4: page_4::Page,
    pub page_5: page_5::Page,
    pub page_6: page_6::Page
}

impl AppWidgets {
    pub fn try_get() -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run.ui");

        let result = Self {
            window: get_object(&builder, "window")?,
            carousel: get_object(&builder, "carousel")?,

            page_1: page_1::Page::new()?,
            page_2: page_2::Page::new()?,
            page_3: page_3::Page::new()?,
            page_4: page_4::Page::new(get_object(&builder, "window")?)?,
            page_5: page_5::Page::new()?,
            page_6: page_6::Page::new()?
        };

        // Add pages to carousel
        result.carousel.append(&result.page_1.page);
        result.carousel.append(&result.page_2.page);
        result.carousel.append(&result.page_3.page);
        result.carousel.append(&result.page_4.page);
        result.carousel.append(&result.page_5.page);
        result.carousel.append(&result.page_6.page);

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
    FirstPageContinue,
    SecondPageCheck,
    ThirdPageContinue,
    FourthPageContinue,
    FifthPageDownload,
    FifthPageContinue,
    Restart,
    Exit
}

impl Actions {
    pub fn into_fn<T: gtk::glib::IsA<gtk::Widget>>(&self, app: &App) -> Box<dyn Fn(&T)> {
        Box::new(clone!(@weak self as action, @strong app => move |_| {
            app.update(action).unwrap();
        }))
    }
}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default)]
pub struct Values;

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
    // values: Rc<Cell<Values>>,
    actions: Rc<Cell<Option<glib::Sender<Actions>>>>
}

impl App {
    /// Create new application
    pub fn new(app: &gtk::Application) -> Result<Self, String> {
        // Get default widgets from ui file and add events to them
        let result = Self {
            widgets: AppWidgets::try_get()?,
            // values: Default::default(),
            actions: Default::default()
        }.init_events().init_actions();

        // Bind app to the window
        result.widgets.window.set_application(Some(app));

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        self.widgets.page_1.continue_button.connect_clicked(Actions::FirstPageContinue.into_fn(&self));
        self.widgets.page_3.continue_button.connect_clicked(Actions::ThirdPageContinue.into_fn(&self));
        self.widgets.page_4.continue_button.connect_clicked(Actions::FourthPageContinue.into_fn(&self));

        self.widgets.page_2.check_button.connect_clicked(Actions::SecondPageCheck.into_fn(&self));
        self.widgets.page_5.download_button.connect_clicked(Actions::FifthPageDownload.into_fn(&self));

        self.widgets.page_6.restart_button.connect_clicked(Actions::Restart.into_fn(&self));

        self.widgets.page_2.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.page_3.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.page_4.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.page_5.exit_button.connect_clicked(Actions::Exit.into_fn(&self));
        self.widgets.page_6.exit_button.connect_clicked(Actions::Exit.into_fn(&self));

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
                Actions::FirstPageContinue => {
                    match Command::new("git").stdout(Stdio::null()).spawn() {
                        Ok(_) => match Command::new("xdelta3").stdout(Stdio::null()).spawn() {
                            Ok(_) => this.widgets.carousel.scroll_to(&this.widgets.page_3.page, true),
                            Err(_) => this.widgets.carousel.scroll_to(&this.widgets.page_2.page, true)
                        },
                        Err(_) => this.widgets.carousel.scroll_to(&this.widgets.page_2.page, true)
                    }
                }

                Actions::SecondPageCheck => {
                    if let Ok(_) = Command::new("git").stdout(Stdio::null()).spawn() {
                        if let Ok(_) = Command::new("xdelta3").stdout(Stdio::null()).spawn() {
                            this.widgets.carousel.scroll_to(&this.widgets.page_3.page, true);
                        }
                    }
                }

                Actions::ThirdPageContinue => {
                    this.widgets.carousel.scroll_to(&this.widgets.page_4.page, true);
                }

                Actions::FourthPageContinue => {
                    config::update_raw(this.widgets.page_4.update_config(config::get().unwrap())).unwrap();

                    this.widgets.carousel.scroll_to(&this.widgets.page_5.page, true);
                }

                Actions::FifthPageDownload => {
                    this.widgets.page_5.wine_version.set_sensitive(false);
                    this.widgets.page_5.dxvk_version.set_sensitive(false);

                    this.widgets.page_5.progress_bar.show();

                    let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);

                    let progress_bar = this.widgets.page_5.progress_bar.clone();

                    let wine_version = this.widgets.page_5.get_wine_version().clone();
                    let dxvk_version = this.widgets.page_5.get_dxvk_version().clone();

                    let wine_version_copy = wine_version.clone();

                    // Download wine
                    std::thread::spawn(move || {
                        let config = config::get().unwrap();

                        let mut wine_version_installer = Installer::new(&wine_version_copy.uri).unwrap();

                        if let Some(temp_folder) = config.launcher.temp {
                            wine_version_installer.temp_folder = temp_folder;
                        }

                        wine_version_installer.install(&config.game.wine.builds, move |state| {
                            sender.send(state).unwrap();
                        });
                    });

                    let this = this.clone();

                    // Download wine (had to do so this way)
                    receiver.attach(None, move |state| {
                        match progress_bar.update_from_state(state) {
                            ProgressUpdateResult::Updated => (),
                            ProgressUpdateResult::Error(_, _) => todo!(),

                            ProgressUpdateResult::Finished => {
                                let mut config = config::get().unwrap();
                                let prefix = WinePrefix::new(&config.game.wine.prefix);

                                // Update wine config
                                config.game.wine.selected = Some(wine_version.name.clone());

                                config::update_raw(config.clone()).unwrap();

                                // Create wine prefix
                                prefix.update(&config.game.wine.builds, wine_version.clone()).unwrap();

                                // Prepare DXVK downloader
                                let mut dxvk_version_installer = Installer::new(&dxvk_version.uri).unwrap();

                                if let Some(temp_folder) = config.launcher.temp {
                                    dxvk_version_installer.temp_folder = temp_folder;
                                }

                                let dxvk_version = dxvk_version.clone();
                                let progress_bar = progress_bar.clone();

                                let this = this.clone();

                                // Download DXVK
                                dxvk_version_installer.install(&config.game.dxvk.builds, move |state| {
                                    match progress_bar.update_from_state(state) {
                                        ProgressUpdateResult::Updated => (),
                                        ProgressUpdateResult::Error(_, _) => todo!(),
            
                                        ProgressUpdateResult::Finished => {
                                            let mut config = config::get().unwrap();

                                            // Apply DXVK
                                            println!("{}", dxvk_version.apply(&config.game.dxvk.builds, &config.game.wine.prefix).unwrap());

                                            // Update dxvk config
                                            config.game.dxvk.selected = Some(dxvk_version.name.clone());

                                            config::update_raw(config.clone()).unwrap();

                                            // Remove .first-run file
                                            let launcher_dir = crate::lib::consts::launcher_dir().unwrap();
                                            std::fs::remove_file(format!("{}/.first-run", launcher_dir)).unwrap();

                                            // Show next page
                                            this.update(Actions::FifthPageContinue).unwrap();
                                        }
                                    }
                                });
                            }
                        }

                        glib::Continue(true)
                    });
                }

                Actions::FifthPageContinue => {
                    this.widgets.carousel.scroll_to(&this.widgets.page_6.page, true);
                }

                Actions::Restart => {
                    Command::new(std::env::current_exe().unwrap()).spawn().unwrap();

                    this.widgets.window.close();
                }

                Actions::Exit => {
                    this.widgets.window.close();
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

unsafe impl Send for App {}
