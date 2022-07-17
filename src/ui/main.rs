use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk4::glib;
use gtk4::glib::clone;

use std::rc::Rc;
use std::cell::Cell;

use anime_game_core::prelude::*;

use crate::ui::*;

use super::preferences::PreferencesStack;
use super::traits::toast_error::ToastError;

use crate::lib::config;
use crate::lib::game;
use crate::lib::tasks;
use crate::lib::launcher::states::LauncherState;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub window: adw::ApplicationWindow,
    pub toast_overlay: adw::ToastOverlay,

    pub leaflet: adw::Leaflet,
    pub status_page: adw::StatusPage,
    pub launcher_content: adw::PreferencesPage,

    pub launch_game: gtk::Button,
    pub open_preferences: gtk::Button,

    pub launch_game_group: adw::PreferencesGroup,
    pub progress_bar_group: adw::PreferencesGroup,
    pub progress_bar: gtk::ProgressBar,

    pub preferences_stack: PreferencesStack
}

impl AppWidgets {
    fn try_get() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../assets/ui/.dist/main.ui"));

        let window = get_object::<adw::ApplicationWindow>(&builder, "window")?;
        let toast_overlay = get_object::<adw::ToastOverlay>(&builder, "toast_overlay")?;

        let result = Self {
            window: window.clone(),
            toast_overlay: toast_overlay.clone(),

            leaflet: get_object(&builder, "leaflet")?,
            status_page: get_object(&builder, "status_page")?,
            launcher_content: get_object(&builder, "launcher_content")?,

            launch_game: get_object(&builder, "launch_game")?,
            open_preferences: get_object(&builder, "open_preferences")?,

            launch_game_group: get_object(&builder, "launch_game_group")?,
            progress_bar_group: get_object(&builder, "progress_bar_group")?,
            progress_bar: get_object(&builder, "progress_bar")?,

            preferences_stack: PreferencesStack::new(window, toast_overlay)?
        };

        // Add preferences page to the leaflet
        result.leaflet.append(&result.preferences_stack.preferences).set_name(Some("preferences_page"));

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug, glib::Downgrade)]
pub enum Actions {
    OpenPreferencesPage,
    PreferencesGoBack,
    PerformButtonEvent,
    DownloadDiff(Rc<VersionDiff>),
    ShowProgressBar,
    UpdateProgress { fraction: Rc<f64>, title: Rc<String> },
    HideProgressBar
}

impl Actions {
    pub fn into_fn<T: gtk::glib::IsA<gtk::Widget>>(&self, app: &App) -> Box<dyn Fn(&T)> {
        Box::new(clone!(@weak self as action, @strong app => move |_| {
            app.update(action);
        }))
    }
}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default, glib::Downgrade)]
pub struct Values {
    state: Rc<LauncherState>
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
#[derive(Clone, glib::Downgrade)]
pub struct App {
    widgets: AppWidgets,
    values: Rc<Cell<Values>>,
    actions: Rc<Cell<Option<glib::Sender<Actions>>>>
}

impl App {
    /// Create new application
    pub fn new(app: &gtk::Application) -> Result<Self, String> {
        let result = Self {
            widgets: AppWidgets::try_get()?,
            values: Default::default(),
            actions: Default::default()
        }.init_events().init_actions();

        // Bind app to the window
        result.widgets.window.set_application(Some(app));

        // Load initial launcher state
        std::thread::spawn(clone!(@strong result => move || {
            match LauncherState::get(Some(&result.widgets.status_page)) {
                Ok(state) => {
                    result.set_state(state);

                    result.widgets.status_page.hide();
                    result.widgets.launcher_content.show();
                },
                Err(err) => {
                    glib::MainContext::default().invoke(move || {
                        result.toast_error("Failed to get initial launcher state", err);
                    });
                }
            }
        }));

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Open preferences page
        self.widgets.open_preferences.connect_clicked(Actions::OpenPreferencesPage.into_fn(&self));

        // Go back button for preferences page
        self.widgets.preferences_stack.preferences_go_back.connect_clicked(Actions::PreferencesGoBack.into_fn(&self));

        // Launch game
        self.widgets.launch_game.connect_clicked(Actions::PerformButtonEvent.into_fn(&self));

        self
    }

    /// Add actions processors
    /// 
    /// Changes will happen in the main thread so you can call `update` method from separate thread
    pub fn init_actions(self) -> Self {
        let (sender, receiver) = glib::MainContext::channel::<Actions>(glib::PRIORITY_DEFAULT);

        // I prefer to avoid using clone! here because it breaks my code autocompletion
        let this = self.clone();

        receiver.attach(None, move |action| {
            // Some debug output
            println!("[main] [update] action: {:?}", &action);

            match action {
                Actions::OpenPreferencesPage => {
                    this.widgets.leaflet.set_visible_child_name("preferences_page");

                    tasks::run(clone!(@strong this => async move {
                        if let Err(err) = this.widgets.preferences_stack.update() {
                            glib::MainContext::default().invoke(move || {
                                this.update(Actions::PreferencesGoBack);
                                this.toast_error("Failed to update preferences", err);
                            });
                        }
                    }));
                }

                Actions::PreferencesGoBack => {
                    this.widgets.leaflet.navigate(adw::NavigationDirection::Back);
                }

                Actions::PerformButtonEvent => {
                    let values = this.values.take();
                    let state = (*values.state).clone();

                    this.values.set(values);

                    match state {
                        LauncherState::Launch => {
                            // Display toast message if the game is failed to run
                            if let Err(err) = game::run(false) {
                                this.toast_error("Failed to run game", err);
                            }
                        },

                        LauncherState::PatchAvailable(_) => todo!(),

                        LauncherState::VoiceUpdateAvailable(diff) |
                        LauncherState::VoiceNotInstalled(diff) |
                        LauncherState::GameUpdateAvailable(diff) |
                        LauncherState::GameNotInstalled(diff) => {
                            this.update(Actions::DownloadDiff(Rc::new(diff)));
                        },

                        LauncherState::GameOutdated(_) => (),
                        LauncherState::VoiceOutdated(_) => ()
                    }
                }
                
                Actions::DownloadDiff(diff) => {
                    match config::get() {
                        Ok(config) => {
                            fn to_gb(bytes: u64) -> f64 {
                                (bytes as f64 / 1024.0 / 1024.0 / 1024.0 * 100.0).ceil() / 100.0
                            }

                            let diff = (*diff).clone();

                            std::thread::spawn(clone!(@strong this => move || {
                                diff.install_to(config.game.path, clone!(@strong this => move |state| {
                                    match state {
                                        InstallerUpdate::DownloadingStarted(_) => {
                                            this.update(Actions::ShowProgressBar);

                                            this.update(Actions::UpdateProgress {
                                                fraction: Rc::new(0.0),
                                                title: Rc::new(String::from("Downloading..."))
                                            });
                                        }

                                        InstallerUpdate::DownloadingProgress(curr, total) => {
                                            // To reduce amount of action requests
                                            if curr % 10000 < 200 {
                                                let progress = curr as f64 / total as f64;

                                                this.update(Actions::UpdateProgress {
                                                    fraction: Rc::new(progress),
                                                    title: Rc::new(format!(
                                                        "Downloading: {:.2}% ({} of {} GB)",
                                                        progress * 100.0,
                                                        to_gb(curr),
                                                        to_gb(total)
                                                    ))
                                                });
                                            }
                                        }

                                        InstallerUpdate::UnpackingStarted(_) => {
                                            this.update(Actions::UpdateProgress {
                                                fraction: Rc::new(0.0),
                                                title: Rc::new(String::from("Unpacking..."))
                                            });
                                        }

                                        InstallerUpdate::UnpackingProgress(curr, total) => {
                                            let progress = curr as f64 / total as f64;

                                            this.update(Actions::UpdateProgress {
                                                fraction: Rc::new(progress),
                                                title: Rc::new(format!(
                                                    "Unpacking: {:.2}% ({} of {} GB)",
                                                    progress * 100.0,
                                                    to_gb(curr),
                                                    to_gb(total)
                                                ))
                                            });
                                        }

                                        InstallerUpdate::DownloadingFinished => (),

                                        InstallerUpdate::UnpackingFinished => {
                                            this.update(Actions::HideProgressBar);
                                        }

                                        InstallerUpdate::DownloadingError(err) => this.toast_error("Failed to download game", err),
                                        InstallerUpdate::UnpackingError => this.toast_error("Failed to unpack game", "?")
                                    }
                                })).unwrap();
                            }));
                        },
                        Err(err) => {
                            glib::MainContext::default().invoke(clone!(@strong this => move || {
                                this.toast_error("Failed to load config", err);
                            }));
                        }
                    }
                }

                Actions::ShowProgressBar => {
                    this.widgets.progress_bar.set_text(None);
                    this.widgets.progress_bar.set_fraction(0.0);

                    this.widgets.launch_game_group.hide();
                    this.widgets.progress_bar_group.show();
                }

                Actions::UpdateProgress { fraction, title } => {
                    this.widgets.progress_bar.set_text(Some(title.as_str()));
                    this.widgets.progress_bar.set_fraction(*fraction);
                }

                Actions::HideProgressBar => {
                    this.widgets.launch_game_group.show();
                    this.widgets.progress_bar_group.hide();
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

    pub fn set_state(&self, state: LauncherState) {
        println!("[main] [set_state] state: {:?}", &state);

        match state {
            LauncherState::Launch => {
                self.widgets.launch_game.set_label("Launch");
            }

            LauncherState::PatchAvailable(_) => {
                self.widgets.launch_game.set_label("Apply patch");
            }

            LauncherState::GameUpdateAvailable(_) |
            LauncherState::VoiceUpdateAvailable(_) => {
                self.widgets.launch_game.set_label("Update");
            }

            LauncherState::GameNotInstalled(_) |
            LauncherState::VoiceNotInstalled(_) => {
                self.widgets.launch_game.set_label("Download");
            }

            LauncherState::VoiceOutdated(_) |
            LauncherState::GameOutdated(_) => {
                self.widgets.launch_game.set_label("Update");
                self.widgets.launch_game.set_tooltip_text(Some("Version is too outdated and can't be updated"));
                self.widgets.launch_game.set_sensitive(false);
            }
        }

        let mut values = self.values.take();

        values.state = Rc::new(state);

        self.values.set(values);
    }
}

impl ToastError for App {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay) {
        (self.widgets.window.clone(), self.widgets.toast_overlay.clone())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
