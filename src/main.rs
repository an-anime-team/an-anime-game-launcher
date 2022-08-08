use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

use gtk::{CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk::gdk::Display;
use gtk::glib;
use gtk::glib::clone;

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
    glib::set_application_name("An Anime Game Launcher");
    glib::set_program_name(Some("An Anime Game Launcher"));

    // Create app
    let application = gtk::Application::new(
        Some(APP_ID),
        Default::default()
    );

    application.add_main_option(
        "run-game", 
        glib::Char::from(0),
        glib::OptionFlags::empty(),
        glib::OptionArg::None,
        "Run the game",
        None
    );

    let run_game = std::rc::Rc::new(std::cell::Cell::new(false));

    application.connect_handle_local_options(clone!(@strong run_game => move |_, arg| {
        if arg.contains("run-game") {
            run_game.set(true);
        }

        -1
    }));

    // Init app window and show it
    application.connect_activate(move |app| {
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

        else {
            // Set game edition
            let config = lib::config::get().expect("Failed to load config");

            anime_game_core::consts::set_game_edition(config.launcher.edition.into());

            // Load main window
            let main = MainApp::new(app).expect("Failed to init MainApp");

            // Load initial launcher state
            let awaiter = main.update_state();

            if !run_game.get() {
                main.show();
            }

            else {
                awaiter.then(move |state| {
                    match state.as_ref().expect("Failed to load launcher state") {
                        lib::launcher::states::LauncherState::Launch => {
                            main.update(ui::main::Actions::PerformButtonEvent).unwrap();

                            std::process::exit(0);
                        },
                        _ => main.show()
                    }
                });
            }
        }
    });

    // Flush config from the memory to the file before closing the app
    application.connect_shutdown(|_| {
        lib::config::flush().expect("Failed to save config file");
    });

    // Run app
    application.run();
}
