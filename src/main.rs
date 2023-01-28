use gtk::gdk::Display;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};

use std::fs;
use std::path::Path;

pub mod lib;
pub mod ui;

use ui::*;

pub const APP_ID: &str = "moe.launcher.an-anime-game-launcher-gtk";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DEBUG: bool = cfg!(debug_assertions);

fn main() {
    adw::init().expect("Libadwaita initialization failed");

    // Register and include resources
    gtk::gio::resources_register_include!(".assets.gresource")
        .expect("Failed to register resources");

    // Set application's title
    glib::set_application_name("An Anime Game Launcher");
    glib::set_program_name(Some("An Anime Game Launcher"));

    // Create app
    let application = gtk::Application::new(Some(APP_ID), Default::default());

    application.add_main_option(
        "run-game",
        glib::Char::from(0),
        glib::OptionFlags::empty(),
        glib::OptionArg::None,
        "Run the game",
        None
    );

    application.add_main_option(
        "just-run-game",
        glib::Char::from(0),
        glib::OptionFlags::empty(),
        glib::OptionArg::None,
        "Run the game whenever it possible, ignoring updates predownloads",
        None
    );

    let run_game = std::rc::Rc::new(std::cell::Cell::new(false));
    let just_run_game = std::rc::Rc::new(std::cell::Cell::new(false));

    application.connect_handle_local_options(
        clone!(@strong run_game, @strong just_run_game => move |_, arg| {
            if arg.contains("just-run-game") {
                just_run_game.set(true);
            }

            else if arg.contains("run-game") {
                run_game.set(true);
            }

            -1
        }),
    );

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

        if !launcher_dir.exists() || launcher_dir.join(".first-run").exists() {
            fs::create_dir_all(&launcher_dir).expect("Failed to create default launcher dir");
            fs::write(launcher_dir.join(".first-run"), "").expect("Failed to create .first-run file");

            let first_run = FirstRunApp::new(app).expect("Failed to init FirstRunApp");

            first_run.show();
        }

        else {
            let config = lib::config::get().expect("Failed to load config");

            // Create wine builds folder
            if !Path::new(&config.game.wine.builds).exists() {
                fs::create_dir_all(config.game.wine.builds)
                    .expect("Failed to create wine builds directory");
            }

            // Create DXVK builds folder
            if !Path::new(&config.game.dxvk.builds).exists() {
                fs::create_dir_all(config.game.dxvk.builds)
                    .expect("Failed to create DXVK builds directory");
            }

            // Set game edition
            anime_game_core::genshin::consts::set_game_edition(config.launcher.edition.into());

            // Load main window
            let main = MainApp::new(app).expect("Failed to init MainApp");

            // Load initial launcher state
            let awaiter = main.update_state();

            if !run_game.get() && !just_run_game.get() {
                main.show();
            }

            else {
                use lib::launcher::states::LauncherState;

                let just_run_game = just_run_game.get();

                awaiter.then(move |state| {
                    let mut state = state.as_ref().expect("Failed to load launcher state");

                    #[allow(clippy::or_fun_call)]
                    if let LauncherState::PredownloadAvailable { game, voices } = state {
                        if just_run_game {
                            state = &LauncherState::Launch;
                        }

                        else if let Ok(config) = lib::config::get() {
                            let mut predownloaded = true;

                            let temp = config.launcher.temp.unwrap_or("/tmp".into());

                            if !temp.join(game.file_name().unwrap_or(String::from("\0"))).exists() {
                                predownloaded = false;
                            }

                            else {
                                for voice in voices {
                                    if !temp.join(voice.file_name().unwrap_or(String::from("\0"))).exists() {
                                        predownloaded = false;

                                        break;
                                    }
                                }
                            }

                            if predownloaded {
                                state = &LauncherState::Launch;
                            }
                        }
                    }

                    match state {
                        LauncherState::Launch => {
                            main.update(ui::main::Actions::PerformButtonEvent).unwrap();

                            std::thread::sleep(std::time::Duration::from_secs(5));
                            std::process::exit(0);
                        }

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
