use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

pub mod ui;
pub mod lib;

use ui::*;

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
        let app = MainApp::new(app).unwrap();

        let app_copy = app.clone();

        app.open_preferences.connect_clicked(move |_| {
            if let Err(err) = app_copy.open_preferences_page() {
                app_copy.toast_error("Failed to open settings page", err);
            }
        });

        app.window.show();
    });

    // Run app
    application.run();
}
