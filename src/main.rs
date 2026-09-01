use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use relm4::prelude::*;
use anime_launcher_sdk::config::ConfigExt;
use anime_launcher_sdk::genshin::config::{Config, Schema};
use anime_launcher_sdk::genshin::states::LauncherState;
use anime_launcher_sdk::genshin::consts::*;
use anime_launcher_sdk::anime_game_core::prelude::*;
use anime_launcher_sdk::anime_game_core::genshin::prelude::*;
use anime_launcher_sdk::sessions::SessionsExt;
use anime_launcher_sdk::genshin::sessions::Sessions;
use tracing_subscriber::prelude::*;
use tracing_subscriber::filter::*;

pub mod move_files;
pub mod i18n;
pub mod background;
pub mod ui;

use ui::main::*;
use ui::first_run::main::*;

pub const APP_ID: &str = "moe.launcher.an-anime-game-launcher";
pub const APP_RESOURCE_PATH: &str = "/moe/launcher/an-anime-game-launcher";

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DEBUG: bool = cfg!(debug_assertions);

/// Sets to `true` when the `App` component is ready (fully initialized)
pub static READY: AtomicBool = AtomicBool::new(false);

// TODO: get rid of using this function in all the components' events
//       e.g. by converting preferences pages into Relm4 Components
/// Check if the app is ready
pub fn is_ready() -> bool {
    READY.load(Ordering::Relaxed)
}

/// Check if a Wayland compositor is available by looking at WAYLAND_DISPLAY
/// first, then falling back to the wayland-0 socket in XDG_RUNTIME_DIR.
pub fn is_wayland_available() -> bool {
    if std::env::var_os("WAYLAND_DISPLAY").is_some() {
        return true;
    }

    let Some(runtime_dir) = std::env::var_os("XDG_RUNTIME_DIR") else {
        return false;
    };

    std::path::Path::new(&runtime_dir).join("wayland-0").exists()
}

lazy_static::lazy_static! {
    /// Config loaded on the app's start. Use `Config::get()` to get up to date config instead.
    /// This one is used to prepare some launcher UI components on start
    pub static ref CONFIG: Schema = Config::get().expect("Failed to load config");

    pub static ref GAME: Game = Game::new(CONFIG.game.path.for_edition(CONFIG.launcher.edition), CONFIG.launcher.edition);

    /// Path to launcher folder. Standard is `$HOME/.local/share/anime-game-launcher`
    pub static ref LAUNCHER_FOLDER: PathBuf = launcher_dir().expect("Failed to get launcher folder");

    /// Path to launcher's cache folder. Standard is `$HOME/.cache/anime-game-launcher`
    pub static ref CACHE_FOLDER: PathBuf = cache_dir().expect("Failed to get launcher's cache folder");

    /// Path to `debug.log` file. Standard is `$HOME/.local/share/anime-game-launcher/debug.log`
    pub static ref DEBUG_FILE: PathBuf = LAUNCHER_FOLDER.join("debug.log");

    /// Path to `background` file. Standard is `$HOME/.local/share/anime-game-launcher/background`
    pub static ref BACKGROUND_FILE: PathBuf = LAUNCHER_FOLDER.join("background");

    /// Path to `background-overlat` file. Standard is `$HOME/.local/share/anime-game-launcher/background-overlay`
    pub static ref BACKGROUND_OVERLAY_FILE: PathBuf = LAUNCHER_FOLDER.join("background-overlay");

    /// Path to the processed `background` file. Standard is `$HOME/.cache/anime-game-launcher/background`
    pub static ref PROCESSED_BACKGROUND_FILE: PathBuf = CACHE_FOLDER.join("background");

    /// Path to the processed `background-overlay` file. Standard is `$HOME/.cache/anime-game-launcher/background-overlay`
    pub static ref PROCESSED_BACKGROUND_OVERLAY_FILE: PathBuf = CACHE_FOLDER.join("background-overlay");

    /// Path to the processed `background-video` file. Standard is `$HOME/.cache/anime-game-launcher/background-video`
    pub static ref BACKGROUND_VIDEO_FILE: PathBuf = CACHE_FOLDER.join("background-video");

    /// Path to `.keep-background` file. Used to mark launcher that it shouldn't update background picture
    ///
    /// Standard is `$HOME/.local/share/anime-game-launcher/.keep-background`
    pub static ref KEEP_BACKGROUND_FILE: PathBuf = LAUNCHER_FOLDER.join(".keep-background");

    /// Path to `.first-run` file. Used to mark launcher that it should run FirstRun window
    ///
    /// Standard is `$HOME/.local/share/anime-game-launcher/.first-run`
    pub static ref FIRST_RUN_FILE: PathBuf = LAUNCHER_FOLDER.join(".first-run");

    /// Global app's css
    static ref GLOBAL_CSS: String = format!("
        progressbar > text {{
            margin-bottom: 6px;
            font-weight: bold;
        }}

        window.classic-style {{
            background: url(\"file://{}\"), url(\"file://{}\");
            background-repeat: no-repeat, no-repeat;
            background-size: cover, cover;
        }}

        .background-overlay {{
            background: url(\"file://{}\");
            background-repeat: no-repeat;
            background-size: cover;
        }}

        window.classic-style progressbar {{
            background-color: rgba(0, 0, 0, 0.65);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 18px;
            padding: 10px 18px;
            box-shadow: 0 2px 12px rgba(0, 0, 0, 0.35);
            color: #ffffff;
            transition-duration: 0.3s;
            transition-timing-function: ease;
        }}

        window.classic-style progressbar:hover {{
            background-color: rgba(0, 0, 0, 0.8);
        }}

        window.classic-style progressbar > text {{
            color: #ffffff;
            text-shadow: 0 1px 3px rgba(0, 0, 0, 0.9);
        }}

        window.classic-style progressbar > trough {{
            min-height: 10px;
            border: none;
            border-radius: 999px;
            background-color: rgba(255, 255, 255, 0.22);
            box-shadow: none;
        }}

        window.classic-style progressbar > trough > progress {{
            min-height: 10px;
            margin: 0;
            border: none;
            border-radius: 999px;
            box-shadow: none;
        }}

        .round-bin {{
            border-radius: 24px;
        }}
        ",
        PROCESSED_BACKGROUND_OVERLAY_FILE.to_string_lossy(),
        PROCESSED_BACKGROUND_FILE.to_string_lossy(),
        PROCESSED_BACKGROUND_OVERLAY_FILE.to_string_lossy(),
        );
}

/// Actually document the launcher's command line options
fn print_help() {
    println!(r#"
An Anime Game Launcher {APP_VERSION}

Usage:
  anime-game-launcher [OPTION...]

Options:
  -h, --help            Show this help message and exit
  --debug               Force debug output in stdout
  --no-verbose-tracing  Disable verbose tracing output in stdout
  --run-game            Launch the game right away if it's ready to run,
                        otherwise open the launcher window
  --just-run-game       Same as --run-game, but also launches the game when
                        an update is available for predownload
  --session <NAME>      Switch to the given session before starting

GTK options are supported as well. Use --help-all to list them.
    "#);
}

fn main() -> anyhow::Result<()> {
    // Setup custom panic handler
    human_panic::setup_panic!(human_panic::metadata!());

    // Print the help message before doing anything else so that it doesn't get
    // mixed with the tracing output, and doesn't create the launcher folders
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        print_help();

        return Ok(());
    }

    // Create launcher folder if it doesn't exist.
    if !LAUNCHER_FOLDER.exists() {
        // check if the location is a symlink. [Path::exists] resolves the symlink and
        // returns whether its *target* exists or not.
        if LAUNCHER_FOLDER.is_symlink() {
            eprintln!(
                "{} is a broken symlink, meaning the directory it is pointing to does not exist, cannot proceed.",
                LAUNCHER_FOLDER.display()
            );
            anyhow::bail!("Launcher folder is a broken symlink");
        }

        std::fs::create_dir_all(LAUNCHER_FOLDER.as_path())
            .expect("Failed to create launcher folder");

        // This one is kinda critical but well, I can't do anything about it
        std::fs::write(FIRST_RUN_FILE.as_path(), "").expect("Failed to create .first-run file");

        // Set initial launcher language based on system language
        // CONFIG is initialized lazily so it will contain following changes as well
        let mut config = Config::get().expect("Failed to get config");

        config.launcher.language = i18n::format_lang(i18n::get_default_lang());

        Config::update_raw(config).expect("Failed to update config");
    }

    // Create cache folder if it doesn't exist.
    if !CACHE_FOLDER.exists() {
        if CACHE_FOLDER.is_symlink() {
            eprintln!(
                "{} is a broken symlink, meaning the directory it is pointing to does not exist, cannot proceed.",
                CACHE_FOLDER.display()
            );
            anyhow::bail!("Cache folder is a broken symlink");
        }

        std::fs::create_dir_all(CACHE_FOLDER.as_path()).expect("Failed to create cache folder");
    }

    // Force debug output
    let mut force_debug = 0;

    // Run the game
    let mut run_game = false;

    // Force run the game
    let mut just_run_game = false;

    // Force disable verbose tracing output in stdout
    let mut no_verbose_tracing = false;

    let args = std::env::args().collect::<Vec<_>>();
    let mut gtk_args = Vec::new();

    // Parse arguments
    for i in 0..args.len() {
        match args[i].as_str() {
            "--debug" => force_debug += 1,
            "--run-game" => run_game = true,
            "--just-run-game" => just_run_game = true,
            "--no-verbose-tracing" => no_verbose_tracing = true,

            "--session" => {
                // Switch active session prior running the app
                if let Some(session) = args.get(i + 1) {
                    Sessions::set_current(session.to_owned())?;
                }
            }

            arg => gtk_args.push(arg.to_string())
        }
    }

    // Prepare stdout logger
    let stdout = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter({
            if force_debug >= 2 {
                LevelFilter::TRACE
            } else if APP_DEBUG || force_debug >= 1 {
                LevelFilter::DEBUG
            } else {
                LevelFilter::WARN
            }
        })
        .with_filter(filter_fn(move |metadata| {
            !metadata.target().contains("rustls")
                && !metadata.target().contains("reqwest")
                && !metadata.target().contains("h2")
                && !metadata.target().contains("hyper_util")
                && !no_verbose_tracing
        }));

    // Prepare debug file logger
    let file = std::fs::File::create(DEBUG_FILE.as_path())?;

    let debug_log = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(false)
        .with_writer(std::sync::Arc::new(file))
        .with_filter({
            if force_debug >= 2 {
                LevelFilter::TRACE
            } else {
                LevelFilter::DEBUG
            }
        })
        .with_filter(filter_fn(|metadata| {
            !metadata.target().contains("rustls")
                && !metadata.target().contains("reqwest")
                && !metadata.target().contains("h2")
                && !metadata.target().contains("hyper_util")
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

    // Set icons search path
    gtk::IconTheme::for_display(&gtk::gdk::Display::default().unwrap())
        .add_resource_path(&format!("{APP_RESOURCE_PATH}/icons"));

    // Set global css
    relm4::set_global_css(&GLOBAL_CSS);

    // Set application's title
    gtk::glib::set_application_name("An Anime Game Launcher");
    gtk::glib::set_program_name(Some("An Anime Game Launcher"));

    // Set UI language
    let lang = CONFIG
        .launcher
        .language
        .parse()
        .expect("Wrong language format used in config");

    i18n::set_lang(lang).expect("Failed to set launcher language");

    tracing::info!("Set UI language to {}", i18n::get_lang());

    // Run FirstRun window if .first-run file persist
    if FIRST_RUN_FILE.exists() {
        // Create the app
        let app = RelmApp::new(APP_ID).with_args(gtk_args);

        // Show first run window
        app.run::<FirstRunApp>(());
    }
    // Run the app if everything's ready
    else {
        if run_game || just_run_game {
            let state =
                LauncherState::get_from_config(|_| {}).expect("Failed to get launcher state");

            match state {
                LauncherState::Launch => {
                    anime_launcher_sdk::genshin::game::run().expect("Failed to run the game");

                    return Ok(());
                }

                LauncherState::PredownloadAvailable {
                    ..
                } if just_run_game => {
                    anime_launcher_sdk::genshin::game::run().expect("Failed to run the game");

                    return Ok(());
                }

                _ => ()
            }
        }

        // Create the app
        let app = RelmApp::new(APP_ID).with_args(gtk_args);

        // Show main window
        app.run::<App>(());
    }

    Ok(())
}
