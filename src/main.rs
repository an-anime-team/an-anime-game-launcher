use relm4::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::prelude::*;
use anime_launcher_sdk::consts::launcher_dir;

use tracing_subscriber::prelude::*;
use tracing_subscriber::filter::*;

use std::path::PathBuf;

pub mod i18n;
pub mod ui;
pub mod background;

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
    /// Config loaded on the app's start. Use `config::get()` to get up to date config instead.
    /// This one is used to prepare some launcher UI components on start
    pub static ref CONFIG: config::Config = config::get().expect("Failed to load config");

    pub static ref GAME: Game = Game::new(&CONFIG.game.path);

    /// Path to launcher folder. Standard is `$HOME/.local/share/anime-game-launcher`
    pub static ref LAUNCHER_FOLDER: PathBuf = launcher_dir().unwrap_or_default();

    /// Path to `debug.log` file. Standard is `$HOME/.local/share/anime-game-launcher/debug.log`
    pub static ref DEBUG_FILE: PathBuf = LAUNCHER_FOLDER.join("debug.log");

    /// Path to `background` file. Standard is `$HOME/.local/share/anime-game-launcher/background`
    pub static ref BACKGROUND_FILE: PathBuf = LAUNCHER_FOLDER.join("background");

    /// Path to `.keep-background` file. Used to mark launcher that it shouldn't update background picture
    /// 
    /// Standard is `$HOME/.local/share/anime-game-launcher/.keep-background`
    pub static ref KEEP_BACKGROUND_FILE: PathBuf = LAUNCHER_FOLDER.join(".keep-background");

    /// Path to `.first-run` file. Used to mark launcher that it should run FirstRun window
    /// 
    /// Standard is `$HOME/.local/share/anime-game-launcher/.first-run`
    pub static ref FIRST_RUN_FILE: PathBuf = LAUNCHER_FOLDER.join(".first-run");
}

fn main() {
    // Create launcher folder if it isn't
    if !LAUNCHER_FOLDER.exists() {
        std::fs::create_dir_all(LAUNCHER_FOLDER.as_path()).expect("Failed to create launcher folder");

        // This one is kinda critical buy well, I can't do something with it
        std::fs::write(FIRST_RUN_FILE.as_path(), "").expect("Failed to create .first-run file");

        // Set initial launcher language based on system language
        let mut config = config::get().expect("Failed to get config");

        config.launcher.language = i18n::format_lang(i18n::get_default_lang());

        config::update_raw(config).expect("Failed to update config");
    }

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
        .pretty()
        .with_ansi(false)
        .with_writer(std::sync::Arc::new(file))
        .with_filter(filter_fn(|metadata| {
            !metadata.target().contains("rustls")
        }));

    tracing_subscriber::registry()
        .with(stdout)
        .with(debug_log)
        .init();

    tracing::info!("Starting application ({APP_VERSION})");

    adw::init().expect("Libadwaita initialization failed");

    // Register and include resources
    gtk::gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    // Set application's title
    gtk::glib::set_application_name("An Anime Game Launcher");
    gtk::glib::set_program_name(Some("An Anime Game Launcher"));

    // Create the app
    let app = RelmApp::new(APP_ID);

    // Set global css
    relm4::set_global_css(&format!("
        progressbar > text {{
            margin-bottom: 4px;
        }}

        window.classic-style {{
            background: url(\"file://{}\");
            background-repeat: no-repeat;
            background-size: cover;
        }}

        window.classic-style progressbar {{
            background-color: #00000040;
            border-radius: 16px;
            padding: 8px 16px;
        }}

        window.classic-style progressbar:hover {{
            background-color: #00000090;
            transition-duration: 0.5s;
            transition-timing-function: linear;
        }}
    ", BACKGROUND_FILE.to_string_lossy()));

    // Set UI language
    let lang = config::get().unwrap().launcher.language.parse().expect("Wrong language format used in config");

    i18n::set_lang(lang).expect("Failed to set launcher language");

    tracing::info!("Set UI language to {}", i18n::get_lang());

    // Run FirstRun window if .first-run file persist
    if FIRST_RUN_FILE.exists() {
        app.run::<ui::first_run::main::FirstRunApp>(());
    }

    // Run the app if everything's ready
    else {
        app.run::<ui::main::App>(());
    }
}
