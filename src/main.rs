use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::{CssProvider, StyleContext, gdk::Display, STYLE_PROVIDER_PRIORITY_APPLICATION};

pub mod ui;
pub mod lib;

use ui::*;

// #[tokio::main]
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
        // Apply CSS styles to the application
        let provider = CssProvider::new();

        provider.load_from_data(include_bytes!("../assets/styles.css"));
        
        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display"),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        // Load main window and show it
        let main = MainApp::new(app).expect("Failed to init MainApp");

        main.show();
    });

    // Run app
    application.run();
}
