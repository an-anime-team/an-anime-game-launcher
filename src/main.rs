use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

use gtk::{CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk::gdk::Display;

use std::path::Path;
use std::fs;

pub mod ui;
pub mod lib;

use ui::*;

pub const APP_ID: &str = "moe.launcher.an-anime-game-launcher-gtk";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DEBUG: bool = cfg!(debug_assertions);

fn main() {
    gtk::init().expect("GTK initialization failed");
    adw::init();

    // Register and include resources
    gtk::gio::resources_register_include!(".assets.gresource")
        .expect("Failed to register resources");

    // Set application's title
    gtk::glib::set_application_name("An Anime Game Launcher");
    gtk::glib::set_program_name(Some("An Anime Game Launcher"));

    // Create app
    let application = gtk::Application::new(
        Some(APP_ID),
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

        // Create default launcher folder if needed
        let launcher_dir = lib::consts::launcher_dir().expect("Failed to get launcher dir");

        if !Path::new(&launcher_dir).exists() || Path::new(&format!("{}/.first-run", launcher_dir)).exists() {
            fs::create_dir_all(&launcher_dir).expect("Failed to create default launcher dir");
            fs::write(format!("{}/.first-run", launcher_dir), "").expect("Failed to create .first-run file");

            let first_run = FirstRunApp::new(app).expect("Failed to init FirstRunApp");

            first_run.show();
        }

        // Load main window and show it
        else {
            let main = MainApp::new(app).expect("Failed to init MainApp");

            main.show();
        }
    });

    // Flush config from the memory to the file before closing the app
    application.connect_shutdown(|_| {
        lib::config::flush().expect("Failed to save config file");
    });

    // Run app
    application.run();
}
