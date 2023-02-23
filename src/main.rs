use relm4::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::prelude::*;

use tracing_subscriber::prelude::*;
use tracing_subscriber::filter::*;

use std::path::PathBuf;

pub mod i18n;
pub mod ui;

mod prettify_bytes;

pub use prettify_bytes::prettify_bytes;

pub const APP_ID: &str = "moe.launcher.an-anime-game-launcher-gtk";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DEBUG: bool = cfg!(debug_assertions);

/// Sets to `true` when the `App` component is ready (fully initialized)
pub static mut READY: bool = false;

// TODO: get rid of using this function in all the components' events
//       e.g. by converting preferences pages into Relm4 Components
pub fn is_ready() -> bool {
    unsafe { READY }
}

lazy_static::lazy_static! {
    /// Path to `debug.log` file. Standard is `$HOME/.local/share/anime-game-launcher/debug.log`
    pub static ref DEBUG_FILE: std::path::PathBuf = anime_launcher_sdk::consts::launcher_dir().unwrap_or_default().join("debug.log");

    /// Config loaded on the app's start. Use `config::get()` to get up to date config instead.
    /// This one is used to prepare some launcher UI components on start
    pub static ref CONFIG: config::Config = config::get().expect("Failed to load config");

    pub static ref GAME: Game = Game::new(&CONFIG.game.path);
}

fn main() {
    // Force debug output
    let force_debug = std::env::args().any(|arg| &arg == "--debug");

    // Prepare stdout logger
    let stdout = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter({
            if APP_DEBUG || force_debug {
                LevelFilter::TRACE
            } else {
                LevelFilter::WARN
            }
        })
        .with_filter(filter_fn(|metadata| {
            !metadata.target().contains("rustls")
        }));

    // Prepare debug file logger
    let file = match std::fs::File::create(DEBUG_FILE.as_path()) {
        Ok(file) => file,
        Err(error) => panic!("Failed to create debug.log file: {:?}", error)
    };

    let debug_log = tracing_subscriber::fmt::layer()
        .with_writer(std::sync::Arc::new(file))
        .with_ansi(false)
        .with_filter(filter_fn(|metadata| {
            !metadata.target().contains("rustls")
        }));

    tracing_subscriber::registry()
        .with(stdout)
        .with(debug_log)
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

    // Create the app
    let app = RelmApp::new(APP_ID);

    // Set global css
    relm4::set_global_css(&format!("
        progressbar > text {{
            margin-bottom: 4px;
        }}

        window.classic-style {{
            background: url(\"file://{}/background\");
            background-repeat: no-repeat;
            background-size: cover;
        }}
    ",
        CONFIG.launcher.temp.as_ref().unwrap_or(&PathBuf::from("/tmp")).to_string_lossy()
    ));

    // Run the app
    app.run::<ui::main::App>(());
}
