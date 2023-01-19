use relm4::prelude::*;

use anime_launcher_sdk::config;

pub mod i18n;
pub mod ui;

pub const APP_ID: &str = "moe.launcher.an-anime-game-launcher-gtk";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DEBUG: bool = cfg!(debug_assertions);

fn main() {
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::TRACE)
        .init();

    tracing::info!("Starting application");

    adw::init().expect("Libadwaita initialization failed");

    // Register and include resources
    gtk::gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    // Set application's title
    gtk::glib::set_application_name("An Anime Game Launcher");
    gtk::glib::set_program_name(Some("An Anime Game Launcher"));

    // Set UI language
    unsafe {
        i18n::LANG = config::get().unwrap().launcher.language.parse().unwrap();

        tracing::info!("Set UI language to {}", i18n::LANG);
    }

    // Run the app
    let app = RelmApp::new("moe.launcher.an-anime-game-launcher");

    app.run::<ui::main::App>(());
}
