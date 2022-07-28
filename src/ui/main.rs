use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

use gtk4::glib;
use gtk4::glib::clone;

use std::rc::Rc;
use std::cell::Cell;
use std::io::Error;

use anime_game_core::prelude::*;

use crate::ui::*;

use super::preferences::PreferencesStack;
use super::traits::toast_error::ToastError;
use super::components::progress_bar::*;

use crate::lib::config;
use crate::lib::game;
use crate::lib::tasks;
use crate::lib::launcher::states::LauncherState;
use crate::lib::wine::{
    Version as WineVersion,
    List as WineList
};
use crate::lib::wine_prefix::WinePrefix;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub window: adw::ApplicationWindow,
    pub toast_overlay: adw::ToastOverlay,

    pub menu: gtk::MenuButton,
    pub about: gtk::AboutDialog,

    pub leaflet: adw::Leaflet,
    pub status_page: adw::StatusPage,
    pub launcher_content: adw::PreferencesPage,

    pub launch_game: gtk::Button,
    pub open_preferences: gtk::Button,

    pub progress_bar: ProgressBar,

    pub preferences_stack: PreferencesStack
}

impl AppWidgets {
    pub fn try_get() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../assets/ui/.dist/main.ui"));

        let window = get_object::<adw::ApplicationWindow>(&builder, "window")?;
        let toast_overlay = get_object::<adw::ToastOverlay>(&builder, "toast_overlay")?;

        let result = Self {
            window: window.clone(),
            toast_overlay: toast_overlay.clone(),

            menu: get_object(&builder, "menu")?,
            about: get_object(&builder, "about")?,

            leaflet: get_object(&builder, "leaflet")?,
            status_page: get_object(&builder, "status_page")?,
            launcher_content: get_object(&builder, "launcher_content")?,

            launch_game: get_object(&builder, "launch_game")?,
            open_preferences: get_object(&builder, "open_preferences")?,

            progress_bar: ProgressBar::new(
                get_object(&builder, "progress_bar")?,
                get_object(&builder, "launch_game_group")?,
                get_object(&builder, "progress_bar_group")?
            ),

            preferences_stack: PreferencesStack::new(window, toast_overlay)?
        };

        // Set devel style to ApplicationWindow if it's debug mode
        if crate::APP_DEBUG {
            result.window.add_css_class("devel");
        }

        // Set default About Dialog values
        if crate::APP_DEBUG {
            result.about.set_version(Some(format!("{} (development)", crate::APP_VERSION).as_str()));
        }

        else {
            result.about.set_version(Some(crate::APP_VERSION));
        }

        result.about.set_license_type(gtk::License::Gpl30);

        result.about.set_authors(&[
            "Nikita Podvirnyy <suimin.tu.mu.ga.mi@gmail.com>"
        ]);

        let curl_info = anime_game_core::curl_sys::Version::get();

        result.about.set_system_information(Some(&[
            format!("Anime Game core library version:    {}", anime_game_core::VERSION),
            format!("    Curl version:    {}", curl_info.version()),
            format!("     SSL version:    {}", curl_info.ssl_version().unwrap_or("?")),
            String::new(),
            format!("GTK version:    {}.{}.{}", gtk::major_version(), gtk::minor_version(), gtk::micro_version()),
            format!("Libadwaita version:    {}.{}.{}", adw::major_version(), adw::minor_version(), adw::micro_version()),
            format!("Pango version:    {}", gtk::pango::version_string().unwrap_or("?".into())),
            format!("Cairo version:    {}", gtk::cairo::version_string()),
        ].join("\n")));

        // Add preferences page to the leaflet
        result.leaflet.append(&result.preferences_stack.preferences).set_name(Some("preferences_page"));

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug, Clone, glib::Downgrade)]
pub enum Actions {
    OpenPreferencesPage,
    PreferencesGoBack,
    PerformButtonEvent,
    ShowProgressBar,
    UpdateProgress { fraction: Rc<f64>, title: Rc<String> },
    HideProgressBar,
    ToastError(Rc<(String, Error)>)
}

impl Actions {
    pub fn into_fn<T: gtk::glib::IsA<gtk::Widget>>(&self, app: &App) -> Box<dyn Fn(&T)> {
        Box::new(clone!(@strong self as action, @weak app => move |_| {
            app.update(action.clone()).expect(&format!("Failed to execute action {:?}", &action));
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
        result.update_state();

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        add_action(&self.widgets.menu, "show-about-dialog", clone!(@strong self.widgets.about as about => move || {
            about.show();
        }));

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
            println!("[main] [update] action: {:?}", action);

            match action {
                Actions::OpenPreferencesPage => {
                    this.widgets.leaflet.set_visible_child_name("preferences_page");

                    tasks::run(clone!(@strong this => async move {
                        if let Err(err) = this.widgets.preferences_stack.update() {
                            glib::MainContext::default().invoke(move || {
                                this.update(Actions::PreferencesGoBack).unwrap();

                                this.toast_error("Failed to update preferences", err);
                            });
                        }
                    }));
                }

                Actions::PreferencesGoBack => {
                    this.widgets.leaflet.navigate(adw::NavigationDirection::Back);

                    config::flush().expect("Failed to save config file");
                }

                Actions::PerformButtonEvent => {
                    let values = this.values.take();
                    let state = (*values.state).clone();

                    this.values.set(values);

                    match config::get() {
                        Ok(mut config) => {
                            match state {
                                LauncherState::Launch => {
                                    // Display toast message if the game is failed to run
                                    if let Err(err) = game::run(false) {
                                        this.toast_error("Failed to run game", err);
                                    }
                                },

                                LauncherState::PatchAvailable(_) => todo!(),

                                LauncherState::WineNotInstalled => {
                                    match WineList::list_downloaded(&config.game.wine.builds) {
                                        Ok(list) => {
                                            for version in list {
                                                if version.recommended {
                                                    config.game.wine.selected = Some(version.name);

                                                    config::update(config.clone());

                                                    break;
                                                }
                                            }

                                            if config.game.wine.selected == None {
                                                match WineVersion::latest() {
                                                    Ok(wine) => {
                                                        match Installer::new(wine.uri) {
                                                            Ok(mut installer) => {
                                                                if let Some(temp_folder) = config.launcher.temp {
                                                                    installer.temp_folder = temp_folder;
                                                                }

                                                                let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);
                                                                let this = this.clone();

                                                                this.update(Actions::ShowProgressBar).unwrap();

                                                                // Download wine version
                                                                // We need to update components from the main thread
                                                                receiver.attach(None, move |state| {
                                                                    match this.widgets.progress_bar.update_from_state(state) {
                                                                        ProgressUpdateResult::Updated => (),

                                                                        ProgressUpdateResult::Error(msg, err) => {
                                                                            this.update(Actions::ToastError(Rc::new((msg, err)))).unwrap();
                                                                            this.update(Actions::HideProgressBar).unwrap();
                                                                        }

                                                                        ProgressUpdateResult::Finished => {
                                                                            let mut config = config::get().unwrap();

                                                                            config.game.wine.selected = Some(wine.name.clone());

                                                                            config::update(config);

                                                                            this.update(Actions::HideProgressBar).unwrap();
                                                                            this.update_state();
                                                                        }
                                                                    }

                                                                    glib::Continue(true)
                                                                });

                                                                // Download wine version in separate thread to not to freeze the main one
                                                                std::thread::spawn(move || {
                                                                    installer.install(config.game.wine.builds, move |state| {
                                                                        sender.send(state).unwrap();
                                                                    });
                                                                });
                                                            },
                                                            Err(err) => this.toast_error("Failed to init wine version installer", err)
                                                        }
                                                    },
                                                    Err(err) => this.toast_error("Failed to get latest wine version", err)
                                                }
                                            }

                                            else {
                                                this.update_state();
                                            }
                                        },
                                        Err(err) => this.toast_error("Failed to list downloaded wine versions", err)
                                    }
                                }

                                LauncherState::PrefixNotExists => {
                                    let prefix = WinePrefix::new(&config.game.wine.prefix);

                                    match config.try_get_selected_wine_info() {
                                        Some(wine) => {
                                            let this = this.clone();

                                            std::thread::spawn(move || {
                                                this.widgets.launch_game.set_sensitive(false);

                                                if let Err(err) = prefix.update(&config.game.wine.builds, wine) {
                                                    this.toast_error("Failed to create wine prefix", err);
                                                }

                                                this.widgets.launch_game.set_sensitive(true);

                                                this.update_state();
                                            });
                                        },
                                        None => this.toast_error("Failed to get selected wine version", Error::last_os_error())
                                    }
                                }
        
                                LauncherState::VoiceUpdateAvailable(diff) |
                                LauncherState::VoiceNotInstalled(diff) |
                                LauncherState::GameUpdateAvailable(diff) |
                                LauncherState::GameNotInstalled(diff) => {
                                    let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);
                                    let this = this.clone();
                                    
                                    this.update(Actions::ShowProgressBar).unwrap();

                                    // Download diff
                                    // We need to update components from the main thread
                                    receiver.attach(None, move |state| {
                                        match this.widgets.progress_bar.update_from_state(state) {
                                            ProgressUpdateResult::Updated => (),

                                            ProgressUpdateResult::Error(msg, err) => {
                                                this.update(Actions::ToastError(Rc::new((msg, err)))).unwrap();
                                                this.update(Actions::HideProgressBar).unwrap();
                                            }

                                            ProgressUpdateResult::Finished => {
                                                this.update(Actions::HideProgressBar).unwrap();

                                                this.update_state();
                                            }
                                        }

                                        glib::Continue(true)
                                    });

                                    // Download diff in separate thread to not to freeze the main one
                                    std::thread::spawn(move || {
                                        diff.install_to_by(config.game.path, config.launcher.temp, move |state| {
                                            sender.send(state).unwrap();
                                        }).unwrap();
                                    });
                                },
        
                                LauncherState::GameOutdated(_) => (),
                                LauncherState::VoiceOutdated(_) => ()
                            }
                        },
                        Err(err) => this.toast_error("Failed to load config", err)
                    }
                }

                Actions::ShowProgressBar => {
                    this.widgets.progress_bar.show();
                }

                Actions::UpdateProgress { fraction, title } => {
                    this.widgets.progress_bar.update(*fraction, Some(title.as_str()));
                }

                Actions::HideProgressBar => {
                    this.widgets.progress_bar.hide();
                }

                Actions::ToastError(toast) => {
                    let (msg, err) = (toast.0.clone(), toast.1.to_string());

                    this.toast_error(msg, err);
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

        self.widgets.launch_game.set_tooltip_text(None);
        self.widgets.launch_game.set_sensitive(true);

        match state {
            LauncherState::Launch => {
                self.widgets.launch_game.set_label("Launch");
            }

            LauncherState::PatchAvailable(_) => {
                self.widgets.launch_game.set_label("Apply patch");
            }

            LauncherState::WineNotInstalled => {
                self.widgets.launch_game.set_label("Download wine");
            }

            LauncherState::PrefixNotExists => {
                self.widgets.launch_game.set_label("Create prefix");
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

    pub fn update_state(&self) {
        let this = self.clone();

        this.widgets.status_page.show();
        this.widgets.launcher_content.hide();

        std::thread::spawn(move || {
            match LauncherState::get(Some(&this.widgets.status_page)) {
                Ok(state) => {
                    this.set_state(state);

                    this.widgets.status_page.hide();
                    this.widgets.launcher_content.show();
                },
                Err(err) => {
                    glib::MainContext::default().invoke(move || {
                        this.toast_error("Failed to get initial launcher state", err);
                    });
                }
            }
        });
    }
}

impl ToastError for App {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay) {
        (self.widgets.window.clone(), self.widgets.toast_overlay.clone())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
