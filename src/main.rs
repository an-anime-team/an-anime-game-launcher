use relm4::prelude::*;

use anime_launcher_sdk::config;
use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::prelude::*;

use tracing_subscriber::prelude::*;

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

    // TODO: add loading screen for heavy tasks like this
    //  UPD: tried once. The problem is that I use this variable, as well as ones above,
    //       in the view! macro, which makes it times harder to make the main window load
    //       faster than this variable calculates its value to show StatusPage with loader.
    //       As for now I have no idea how to fix this
    pub static ref GAME_DIFF: Option<VersionDiff> = match GAME.try_get_diff() {
        Ok(diff) => Some(diff),
        Err(err) => {
            tracing::error!("Failed to get game diff {err}");

            None
        }
    };

    pub static ref PATCH: Option<Patch> = match Patch::try_fetch(&CONFIG.patch.servers, None) {
        Ok(patch) => Some(patch),
        Err(err) => {
            tracing::error!("Failed to fetch patch info {err}");

            None
        }
    };
}

fn main() {
    let stdout = tracing_subscriber::fmt::layer().pretty();

    let file = match std::fs::File::create(DEBUG_FILE.as_path()) {
        Ok(file) => file,
        Err(error) => panic!("Failed to create debug.log file: {:?}", error)
    };

    let mut debug_log = tracing_subscriber::fmt::layer()
        .with_writer(std::sync::Arc::new(file));

    debug_log.set_ansi(false);

    tracing_subscriber::registry()
        .with({
            stdout
                .with_filter(tracing_subscriber::filter::LevelFilter::from_level({
                    if APP_DEBUG || std::env::args().any(|arg| &arg == "--debug") {
                        tracing::Level::TRACE
                    } else {
                        tracing::Level::WARN
                    }
                }))
                .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("rustls")
                }))
                .and_then(debug_log)
        })
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
