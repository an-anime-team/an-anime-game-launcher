use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

pub mod ui;

use ui::MainApp;

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

        app.open_preferences.connect_clicked(move |_| {
            app.leaflet.set_visible_child_name("preferences_page");
        });

        app.window.show();
    });

    // Run app
    application.run();
}
