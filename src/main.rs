use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

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
        let app = MainApp::new(app).expect("Failed to init MainApp");

        app.show();
    });

    // Run app
    application.run();
}
