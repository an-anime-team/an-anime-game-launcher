use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

pub mod lib;

struct PreferencesPage {
    pub preferences: gtk::Box,
    pub preferences_go_back: gtk::Button
}

impl PreferencesPage {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_string(include_str!("../ui/.dist/preferences.ui"));

        let result = Self {
            preferences: lib::get_object(&builder, "preferences"),
            preferences_go_back: lib::get_object(&builder, "preferences_go_back")
        };

        result
    }
}

struct App {
    pub window: adw::ApplicationWindow,
    pub leaflet: adw::Leaflet,
    pub launch_game: adw::SplitButton,
    pub open_preferences: gtk::Button
}

impl App {
    pub fn new(app: &gtk::Application) -> Self {
        // Create builder from UI file
        let builder = gtk::Builder::from_string(include_str!("../ui/.dist/main.ui"));

        // Parse objects from builder
        let result = Self {
            window: lib::get_object(&builder, "window"),
            leaflet: lib::get_object(&builder, "leaflet"),
            launch_game: lib::get_object(&builder, "launch_game"),
            open_preferences: lib::get_object(&builder, "open_preferences")
        };

        // Add preferences page to the leaflet
        let page = PreferencesPage::new();
        let leaflet = result.leaflet.clone();

        result.leaflet.append(&page.preferences).set_name(Some("preferences_page"));

        // Go back button for preferences page
        page.preferences_go_back.connect_clicked(move |_| {
            leaflet.navigate(adw::NavigationDirection::Back);
        });

        // Bind app to the window
        result.window.set_application(Some(app));

        result
    }
}

fn main() {
    gtk::init().expect("GTK initialization failed");
    adw::init();

    // Create app
    let application = gtk::Application::new(
        Some("com.gitlab.an-anime-team.an-anime-game-launcher"),
        Default::default()
    );

    // Init app window and show it
    application.connect_activate(|app| {
        let app = App::new(app);

        app.open_preferences.connect_clicked(move |_| {
            app.leaflet.set_visible_child_name("preferences_page");
        });

        app.window.show();
    });

    // Run app
    application.run();
}
