use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use super::get_object;
use super::preferences::PreferencesStack;

pub struct App {
    pub window: adw::ApplicationWindow,
    pub leaflet: adw::Leaflet,
    pub launch_game: adw::SplitButton,
    pub open_preferences: gtk::Button
}

impl App {
    pub fn new(app: &gtk::Application) -> Result<Self, String> {
        // Create builder from UI file
        let builder = gtk::Builder::from_string(include_str!("../../assets/ui/.dist/main.ui"));

        // Parse objects from builder
        let result = Self {
            window: get_object(&builder, "window")?,
            leaflet: get_object(&builder, "leaflet")?,
            launch_game: get_object(&builder, "launch_game")?,
            open_preferences: get_object(&builder, "open_preferences")?
        };

        // Add preferences page to the leaflet
        let stack = PreferencesStack::new()?;
        let leaflet = result.leaflet.clone();

        result.leaflet.append(&stack.preferences).set_name(Some("preferences_page"));

        // Go back button for preferences page
        stack.preferences_go_back.connect_clicked(move |_| {
            leaflet.navigate(adw::NavigationDirection::Back);
        });

        // Bind app to the window
        result.window.set_application(Some(app));

        Ok(result)
    }
}
