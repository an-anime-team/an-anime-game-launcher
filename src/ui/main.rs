use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::Error;

use super::get_object;
use super::preferences::PreferencesStack;
use super::ToastError;

use crate::lib::game;

pub enum AppState {
    Launch,
    Progress {
        title: String,
        progress: f64
    }
}

#[derive(Clone)]
pub struct App {
    pub window: adw::ApplicationWindow,
    pub leaflet: adw::Leaflet,
    pub launch_game: adw::SplitButton,
    pub launch_game_debug: gtk::Button,
    pub open_preferences: gtk::Button,
    pub toast_overlay: adw::ToastOverlay,

    pub launch_game_group: adw::PreferencesGroup,
    pub progress_bar_group: adw::PreferencesGroup,
    pub progress_bar: gtk::ProgressBar,

    pub preferences_stack: PreferencesStack
}

impl App {
    pub fn new(app: &gtk::Application) -> Result<Self, String> {
        // Create builder from UI file
        let builder = gtk::Builder::from_string(include_str!("../../assets/ui/.dist/main.ui"));

        let window = get_object::<adw::ApplicationWindow>(&builder, "window")?;
        let toast_overlay = get_object::<adw::ToastOverlay>(&builder, "toast_overlay")?;

        // Parse objects from builder
        let result = Self {
            window: window.clone(),
            leaflet: get_object(&builder, "leaflet")?,
            launch_game: get_object(&builder, "launch_game")?,
            launch_game_debug: get_object(&builder, "launch_game_debug")?,
            open_preferences: get_object(&builder, "open_preferences")?,
            toast_overlay: toast_overlay.clone(),

            launch_game_group: get_object(&builder, "launch_game_group")?,
            progress_bar_group: get_object(&builder, "progress_bar_group")?,
            progress_bar: get_object(&builder, "progress_bar")?,

            preferences_stack: PreferencesStack::new(window, toast_overlay)?
        };

        // Add preferences page to the leaflet
        let leaflet = result.leaflet.clone();

        result.leaflet.append(&result.preferences_stack.preferences).set_name(Some("preferences_page"));

        // Go back button for preferences page
        result.preferences_stack.preferences_go_back.connect_clicked(move |_| {
            leaflet.navigate(adw::NavigationDirection::Back);
        });

        // Launch game
        let app_copy = result.clone();
        
        result.launch_game.connect_clicked(move |_| {
            // Display toast message if the game is failed to run
            if let Err(err) = game::run(false) {
                app_copy.toast_error("Failed to run game", err);
            }
        });

        // Launch game in debug mode
        /*let app_copy = result.clone();
        
        result.launch_game_debug.connect_clicked(move |_| {
            // Display toast message if the game is failed to run
            if let Err(err) = game::run(true) {
                app_copy.toast_error("Failed to run game", err);
            }
        });*/

        // Bind app to the window
        result.window.set_application(Some(app));

        Ok(result)
    }

    pub fn open_preferences_page(&self) -> Result<(), Error> {
        self.preferences_stack.update()?;
        
        self.leaflet.set_visible_child_name("preferences_page");

        Ok(())
    }

    pub fn update_state(&self, state: AppState) {
        match state {
            AppState::Launch => {
                self.launch_game_group.set_visible(true);
                self.progress_bar_group.set_visible(false);
            },
            AppState::Progress { title, progress } => {
                self.launch_game_group.set_visible(false);
                self.progress_bar_group.set_visible(true);

                self.progress_bar.set_text(Some(&title));
                self.progress_bar.set_fraction(progress);
            }
        }
    }
}

impl ToastError for App {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay) {
        (self.window.clone(), self.toast_overlay.clone())
    }
}
