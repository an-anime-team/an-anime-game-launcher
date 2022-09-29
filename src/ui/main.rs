use gtk::prelude::*;

use gtk::glib;
use gtk::glib::clone;

use std::rc::Rc;
use std::cell::Cell;
use std::io::Error;
use std::process::{Command, Stdio};
use std::path::Path;

use wait_not_await::Await;

use anime_game_core::prelude::*;
use anime_game_core::genshin::prelude::*;

use wincompatlib::prelude::*;

use crate::ui::*;

use super::preferences::PreferencesStack;
use super::traits::toast::Toast;
use super::components::progress_bar::*;

use crate::lib::consts;
use crate::lib::config;
use crate::lib::game;
use crate::lib::launcher::states::LauncherState;
use crate::lib::wine::{
    Version as WineVersion,
    List as WineList
};
use crate::lib::prettify_bytes::prettify_bytes;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub window: adw::ApplicationWindow,
    pub toast_overlay: adw::ToastOverlay,

    pub menu: gtk::MenuButton,
    pub about: adw::AboutWindow,

    pub leaflet: adw::Leaflet,
    pub status_page: adw::StatusPage,
    pub launcher_content: adw::PreferencesPage,

    pub icon: gtk::Image,
    pub predownload_game: gtk::Button,
    pub launch_game: gtk::Button,
    pub open_preferences: gtk::Button,

    pub progress_bar: ProgressBar,

    pub preferences_stack: PreferencesStack
}

impl AppWidgets {
    pub fn try_get() -> anyhow::Result<Self> {
        let builder = gtk::Builder::from_resource("/org/app/ui/main.ui");

        let window = get_object::<adw::ApplicationWindow>(&builder, "window")?;
        let toast_overlay = get_object::<adw::ToastOverlay>(&builder, "toast_overlay")?;

        let result = Self {
            window: window.clone(),
            toast_overlay,

            menu: get_object(&builder, "menu")?,
            about: get_object(&builder, "about")?,

            leaflet: get_object(&builder, "leaflet")?,
            status_page: get_object(&builder, "status_page")?,
            launcher_content: get_object(&builder, "launcher_content")?,

            icon: get_object(&builder, "icon")?,
            predownload_game: get_object(&builder, "predownload_game")?,
            launch_game: get_object(&builder, "launch_game")?,
            open_preferences: get_object(&builder, "open_preferences")?,

            progress_bar: ProgressBar::new(
                get_object(&builder, "progress_bar")?,
                get_object(&builder, "launch_game_group")?,
                get_object(&builder, "progress_bar_group")?
            ),

            preferences_stack: PreferencesStack::new(&window)?
        };

        // Set devel style to ApplicationWindow if it's debug mode
        if crate::APP_DEBUG {
            result.window.add_css_class("devel");
        }
        
        // Load icon from "icon" file if it exists
        if std::path::Path::new("icon").exists() {
            result.icon.set_from_file(Some("icon"));
        }

        // Set default About Dialog values
        if crate::APP_DEBUG {
            result.about.set_version(&format!("{}-dev", crate::APP_VERSION));
        }

        else {
            result.about.set_version(crate::APP_VERSION);
        }

        result.about.set_license_type(gtk::License::Gpl30);

        result.about.set_developers(&[
            "Nikita Podvirnyy https://github.com/krypt0nn"
        ]);

        result.about.add_credit_section(Some("Logo"), &[
            "@nightany https://pinterest.com/pin/356206651788051017"
        ]);

        result.about.add_credit_section(Some("An Anime Team"), &[
            "@Marie https://github.com/Mar0xy",
            "@lane https://github.com/laurinneff"
        ]);

        let curl_info = anime_game_core::curl_sys::Version::get();

        #[allow(clippy::or_fun_call)]
        result.about.set_debug_info(&[
            format!("Anime Game core library version: {}", anime_game_core::VERSION),
            format!("Curl version: {}", curl_info.version()),
            format!("SSL version: {}", curl_info.ssl_version().unwrap_or("?")),
            String::new(),
            format!("GTK version: {}.{}.{}", gtk::major_version(), gtk::minor_version(), gtk::micro_version()),
            format!("Libadwaita version: {}.{}.{}", adw::major_version(), adw::minor_version(), adw::micro_version()),
            format!("Pango version: {}", gtk::pango::version_string().unwrap_or("?".into())),
            format!("Cairo version: {}", gtk::cairo::version_string()),
        ].join("\n"));

        // Add preferences page to the leaflet
        result.leaflet.append(&result.preferences_stack.preferences).set_name(Some("preferences_page"));

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug, Clone, glib::Downgrade)]
pub enum Actions {
    OpenPreferencesPage,
    PreferencesGoBack,
    PerformButtonEvent,
    PredownloadUpdate,
    RepairGame,
    ShowProgressBar,
    UpdateProgress { fraction: Rc<f64>, title: Rc<String> },
    HideProgressBar,
    Toast(Rc<(String, String)>)
}

impl Actions {
    #[allow(clippy::expect_fun_call, clippy::wrong_self_convention)]
    pub fn into_fn<T: gtk::glib::IsA<gtk::Widget>>(&self, app: &App) -> Box<dyn Fn(&T)> {
        Box::new(clone!(@strong self as action, @weak app => move |_| {
            app.update(action.clone()).expect(&format!("Failed to execute action {:?}", &action));
        }))
    }
}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default)]
pub struct Values {
    state: LauncherState
}

/// The main application structure
/// 
/// `Default` macro automatically calls `AppWidgets::default`, i.e. loads UI file and reference its widgets
/// 
/// `Rc<Cell<Values>>` means this:
/// - `Rc` addeds ability to reference the same value from various clones of the structure.
///   This will guarantee us that inner `Cell<Values>` is the same for all the `App::clone()` values
/// - `Cell` addeds inner mutability to its value, so we can mutate it even without mutable reference.
/// 
/// So we have a shared reference to some value that can be changed without mutable reference.
/// That's what we need and what we use in `App::update` method
#[derive(Clone, glib::Downgrade)]
pub struct App {
    widgets: AppWidgets,
    values: Rc<Cell<Values>>,
    actions: Rc<Cell<Option<glib::Sender<Actions>>>>
}

impl App {
    /// Create new application
    pub fn new(app: &gtk::Application) -> anyhow::Result<Self> {
        let mut result = Self {
            widgets: AppWidgets::try_get()?,
            values: Default::default(),
            actions: Default::default()
        }.init_events().init_actions();

        // Set app reference
        result.widgets.preferences_stack.set_app(result.clone());

        // Bind app to the window
        result.widgets.window.set_application(Some(app));

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Add menu actions
        add_action(&self.widgets.menu, "open-launcher-folder", clone!(@weak self as this => move || {
            if let Some(launcher_dir) = consts::launcher_dir() {
                if let Err(err) = Command::new("xdg-open").arg(launcher_dir).spawn() {
                    this.update(Actions::Toast(Rc::new((
                        String::from("Failed to open launcher folder"), err.to_string()
                    )))).unwrap();
                }
            }
        }));

        add_action(&self.widgets.menu, "open-game-folder", clone!(@weak self as this => move || {
            if let Ok(config) = config::get() {
                if let Err(err) = Command::new("xdg-open").arg(config.game.path).spawn() {
                    this.update(Actions::Toast(Rc::new((
                        String::from("Failed to open game folder"), err.to_string()
                    )))).unwrap();
                }
            }
        }));

        add_action(&self.widgets.menu, "open-config-file", clone!(@weak self as this => move || {
            if let Some(config_file) = consts::config_file() {
                if let Err(err) = Command::new("xdg-open").arg(config_file).spawn() {
                    this.update(Actions::Toast(Rc::new((
                        String::from("Failed to open config file"), err.to_string()
                    )))).unwrap();
                }
            }
        }));

        // Other actions

        add_action(&self.widgets.menu, "show-about-dialog", clone!(@strong self.widgets.about as about => move || {
            about.show();
        }));

        // Open preferences page
        self.widgets.open_preferences.connect_clicked(Actions::OpenPreferencesPage.into_fn(&self));

        // Go back button for preferences page
        self.widgets.preferences_stack.preferences_go_back.connect_clicked(Actions::PreferencesGoBack.into_fn(&self));

        // Predownload update
        self.widgets.predownload_game.connect_clicked(Actions::PredownloadUpdate.into_fn(&self));

        // Launch game
        self.widgets.launch_game.connect_clicked(Actions::PerformButtonEvent.into_fn(&self));

        self
    }

    /// Add actions processors
    /// 
    /// Changes will happen in the main thread so you can call `update` method from separate thread
    pub fn init_actions(self) -> Self {
        let (sender, receiver) = glib::MainContext::channel::<Actions>(glib::PRIORITY_DEFAULT);

        // I prefer to avoid using clone! here because it breaks my code autocompletion
        let this = self.clone();

        receiver.attach(None, move |action| {
            // Some debug output
            match &action {
                Actions::UpdateProgress { .. } => (),
                action => println!("[main] [update] action: {:?}", action)
            }

            match action {
                Actions::OpenPreferencesPage => {
                    this.widgets.leaflet.set_visible_child_name("preferences_page");

                    let this = this.clone();

                    std::thread::spawn(move || {
                        if let Err(err) = this.widgets.preferences_stack.update() {
                            glib::MainContext::default().invoke(move || {
                                this.update(Actions::PreferencesGoBack).unwrap();

                                this.toast("Failed to update preferences", err);
                            });
                        }
                    });
                }

                Actions::PreferencesGoBack => {
                    this.widgets.leaflet.navigate(adw::NavigationDirection::Back);

                    config::flush().expect("Failed to save config file");
                }

                Actions::PerformButtonEvent => {
                    let values = this.values.take();
                    let state = values.state.clone();

                    this.values.set(values);

                    match config::get() {
                        Ok(mut config) => {
                            match state {
                                LauncherState::PatchAvailable(Patch::NotAvailable) |
                                LauncherState::PredownloadAvailable { .. } |
                                LauncherState::Launch => {
                                    let this = this.clone();

                                    this.widgets.window.hide();

                                    std::thread::spawn(move || {
                                        // Display toast message if the game is failed to run
                                        if let Err(err) = game::run() {
                                            this.widgets.window.show();

                                            this.update(Actions::Toast(Rc::new((
                                                String::from("Failed to run game"), err.to_string()
                                            )))).unwrap();
                                        }

                                        else {
                                            std::thread::sleep(std::time::Duration::from_secs(2));

                                            loop {
                                                std::thread::sleep(std::time::Duration::from_secs(3));

                                                match Command::new("ps").arg("-A").stdout(Stdio::piped()).output() {
                                                    Ok(output) => {
                                                        let output = String::from_utf8_lossy(&output.stdout);

                                                        if !output.contains("GenshinImpact.e") && !output.contains("unlocker.exe") {
                                                            break;
                                                        }
                                                    },
                                                    Err(_) => break
                                                }
                                            }

                                            this.widgets.window.show();
                                        }
                                    });
                                },

                                LauncherState::PatchAvailable(patch) => {
                                    match patch {
                                        Patch::NotAvailable |
                                        Patch::Outdated { .. } |
                                        Patch::Preparation { .. } => unreachable!(),

                                        Patch::Testing { version, host, .. } |
                                        Patch::Available { version, host, .. } => {
                                            this.widgets.launch_game.set_sensitive(false);
                                            this.widgets.open_preferences.set_sensitive(false);

                                            let this = this.clone();

                                            std::thread::spawn(move || {
                                                let applier = PatchApplier::new(&config.patch.path);

                                                let mut synced = false;

                                                match applier.is_sync_with(&host) {
                                                    Ok(true) => synced = true,

                                                    Ok(false) => {
                                                        match applier.sync(host) {
                                                            Ok(true) => synced = true,

                                                            Ok(false) => {
                                                                this.update(Actions::Toast(Rc::new((
                                                                    String::from("Failed to sync patch folder"), Error::last_os_error().to_string()
                                                                )))).unwrap();
                                                            }

                                                            Err(err) => {
                                                                this.update(Actions::Toast(Rc::new((
                                                                    String::from("Failed to sync patch folder"), err.to_string()
                                                                )))).unwrap();
                                                            }
                                                        }
                                                    }

                                                    Err(err) => this.update(Actions::Toast(Rc::new((
                                                        String::from("Failed to check patch folder state"), err.to_string()
                                                    )))).unwrap()
                                                }

                                                if synced {
                                                    match applier.apply(&config.game.path, version, config.patch.root) {
                                                        Ok(_) => (),

                                                        Err(err) => {
                                                            this.update(Actions::Toast(Rc::new((
                                                                String::from("Failed to patch game"), err.to_string()
                                                            )))).unwrap();
                                                        }
                                                    }
                                                }

                                                glib::MainContext::default().invoke(move || {
                                                    this.widgets.launch_game.set_sensitive(true);
                                                    this.widgets.open_preferences.set_sensitive(true);

                                                    this.update_state();
                                                });
                                            });
                                        }
                                    }
                                }

                                LauncherState::WineNotInstalled => {
                                    match WineList::list_downloaded(&config.game.wine.builds) {
                                        Ok(list) => {
                                            for version in list {
                                                if version.recommended {
                                                    config.game.wine.selected = Some(version.name);

                                                    config::update(config.clone());

                                                    break;
                                                }
                                            }

                                            if config.game.wine.selected == None {
                                                match WineVersion::latest() {
                                                    Ok(wine) => {
                                                        match Installer::new(wine.uri) {
                                                            Ok(mut installer) => {
                                                                if let Some(temp_folder) = config.launcher.temp {
                                                                    installer.temp_folder = temp_folder;
                                                                }

                                                                installer.downloader
                                                                    .set_downloading_speed(config.launcher.speed_limit)
                                                                    .expect("Failed to set downloading speed limit");

                                                                let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);
                                                                let this = this.clone();

                                                                this.update(Actions::ShowProgressBar).unwrap();

                                                                // Download wine version
                                                                // We need to update components from the main thread
                                                                receiver.attach(None, move |state| {
                                                                    match this.widgets.progress_bar.update_from_state(state) {
                                                                        ProgressUpdateResult::Updated => (),

                                                                        ProgressUpdateResult::Error(msg, err) => {
                                                                            this.widgets.progress_bar.hide();

                                                                            this.toast(msg, err);
                                                                        }

                                                                        ProgressUpdateResult::Finished => {
                                                                            let mut config = config::get().unwrap();

                                                                            config.game.wine.selected = Some(wine.name.clone());

                                                                            config::update(config);

                                                                            this.update_state();
                                                                        }
                                                                    }

                                                                    glib::Continue(true)
                                                                });

                                                                // Download wine version in separate thread to not to freeze the main one
                                                                std::thread::spawn(move || {
                                                                    installer.install(config.game.wine.builds, move |state| {
                                                                        sender.send(state).unwrap();
                                                                    });
                                                                });
                                                            },
                                                            Err(err) => this.toast("Failed to init wine version installer", err)
                                                        }
                                                    },
                                                    Err(err) => this.toast("Failed to get latest wine version", err)
                                                }
                                            }

                                            else {
                                                this.update_state();
                                            }
                                        },
                                        Err(err) => this.toast("Failed to list downloaded wine versions", err)
                                    }
                                }

                                LauncherState::PrefixNotExists => {
                                    match config.try_get_wine_executable() {
                                        Some(wine) => {
                                            let this = this.clone();

                                            std::thread::spawn(move || {
                                                this.widgets.launch_game.set_sensitive(false);

                                                let wine = Wine::from_binary(wine)
                                                    .with_loader(WineLoader::Current)
                                                    .with_arch(WineArch::Win64);

                                                if let Err(err) = wine.update_prefix(&config.game.wine.prefix) {
                                                    this.update(Actions::Toast(Rc::new((
                                                        String::from("Failed to create wine prefix"), err.to_string()
                                                    )))).unwrap();
                                                }

                                                this.widgets.launch_game.set_sensitive(true);

                                                this.update_state();
                                            });
                                        },
                                        None => this.toast("Failed to get selected wine version", Error::last_os_error())
                                    }
                                }

                                LauncherState::VoiceUpdateAvailable(diff) |
                                LauncherState::VoiceNotInstalled(diff) |
                                LauncherState::GameUpdateAvailable(diff) |
                                LauncherState::GameNotInstalled(diff) => {
                                    let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);

                                    let this = this.clone();
                                    let this_copy = this.clone();

                                    this.update(Actions::ShowProgressBar).unwrap();

                                    // Download diff
                                    // We need to update components from the main thread
                                    receiver.attach(None, move |state| {
                                        match this.widgets.progress_bar.update_from_state(state) {
                                            ProgressUpdateResult::Updated => (),

                                            ProgressUpdateResult::Error(msg, err) => {
                                                this.widgets.progress_bar.hide();

                                                this.toast(msg, err);
                                            }

                                            ProgressUpdateResult::Finished => {
                                                this.widgets.progress_bar.hide();

                                                let this = this.clone();

                                                this.update_state().then(move |result| {
                                                    if let Ok(state) = result {
                                                        match state {
                                                            LauncherState::VoiceUpdateAvailable(_) |
                                                            LauncherState::VoiceNotInstalled(_) |
                                                            LauncherState::GameUpdateAvailable(_) |
                                                            LauncherState::GameNotInstalled(_) => {
                                                                this.update(Actions::PerformButtonEvent).unwrap();
                                                            },
                                                            _ => ()
                                                        }
                                                    }
                                                });
                                            }
                                        }

                                        glib::Continue(true)
                                    });

                                    // Download diff in separate thread to not to freeze the main one
                                    std::thread::spawn(move || {
                                        let result = diff.install_to_by(config.game.path, config.launcher.temp, move |state| {
                                            sender.send(state).unwrap();
                                        });

                                        if let Err(err) = result {
                                            let err: Error = err.into();

                                            this_copy.update(Actions::Toast(Rc::new((
                                                String::from("Downloading failed"), err.to_string()
                                            )))).unwrap();
                                        }
                                    });
                                },
        
                                LauncherState::GameOutdated(_) => (),
                                LauncherState::VoiceOutdated(_) => ()
                            }
                        },
                        Err(err) => this.toast("Failed to load config", err)
                    }
                }

                Actions::PredownloadUpdate => {
                    let values = this.values.take();
                    let state = values.state.clone();

                    this.values.set(values);

                    match config::get() {
                        Ok(config) => {
                            match state {
                                LauncherState::PredownloadAvailable { game, mut voices } => {
                                    let (sender, receiver) = glib::MainContext::channel::<InstallerUpdate>(glib::PRIORITY_DEFAULT);

                                    let mut diffs: Vec<VersionDiff> = vec![game];

                                    diffs.append(&mut voices);

                                    this.widgets.progress_bar.show();

                                    std::thread::spawn(clone!(@strong this => move || {
                                        for mut diff in diffs {
                                            let sender = sender.clone();
    
                                            #[allow(unused_must_use)]
                                            let result = diff.download_in(config.launcher.temp.as_ref().unwrap(), move |curr, total| {
                                                sender.send(InstallerUpdate::DownloadingProgress(curr, total));
                                            });
    
                                            if let Err(err) = result {
                                                let err: Error = err.into();
    
                                                this.update(Actions::Toast(Rc::new((
                                                    String::from("Downloading failed"), err.to_string()
                                                )))).unwrap();

                                                break;
                                            }
                                        }

                                        this.update(Actions::HideProgressBar).unwrap();
                                    }));

                                    receiver.attach(None, clone!(@strong this => move |state| {
                                        this.widgets.progress_bar.update_from_state(state);

                                        glib::Continue(true)
                                    }));
                                }

                                _ => unreachable!()
                            }
                        },
                        Err(err) => this.toast("Failed to load config", err)
                    }
                }

                Actions::RepairGame => {
                    match config::get() {
                        Ok(config) => {
                            let this = this.clone();

                            std::thread::spawn(move || {
                                match repairer::try_get_integrity_files(None) {
                                    Ok(mut files) => {
                                        // Add voiceovers files
                                        let game = Game::new(&config.game.path);

                                        if let Ok(voiceovers) = game.get_voice_packages() {
                                            for package in voiceovers {
                                                if let Ok(mut voiceover_files) = repairer::try_get_voice_integrity_files(package.locale(), None) {
                                                    files.append(&mut voiceover_files);
                                                }
                                            }
                                        }

                                        this.update(Actions::ShowProgressBar).unwrap();

                                        this.update(Actions::UpdateProgress {
                                            fraction: Rc::new(0.0),
                                            title: Rc::new(String::from("Verifying files: 0%"))
                                        }).unwrap();

                                        const VERIFIER_THREADS_NUM: u64 = 4;

                                        let mut total = 0;

                                        for file in &files {
                                            total += file.size;
                                        }

                                        let median_size = total / VERIFIER_THREADS_NUM;
                                        let mut i = 0;

                                        let (sender, receiver) = std::sync::mpsc::channel();

                                        for _ in 0..VERIFIER_THREADS_NUM {
                                            let mut thread_files = Vec::new();
                                            let mut thread_files_size = 0;

                                            while i < files.len() {
                                                thread_files.push(files[i].clone());

                                                thread_files_size += files[i].size;
                                                i += 1;

                                                if thread_files_size >= median_size {
                                                    break;
                                                }
                                            }

                                            let game_path = config.game.path.clone();
                                            let thread_sender = sender.clone();

                                            std::thread::spawn(move || {
                                                for file in thread_files {
                                                    let status = if config.launcher.repairer.fast {
                                                        file.fast_verify(&game_path)
                                                    } else {
                                                        file.verify(&game_path)
                                                    };

                                                    thread_sender.send((file, status)).unwrap();
                                                }
                                            });
                                        }

                                        // We have VERIFIER_THREADS_NUM copies of this sender + the original one
                                        // receiver will return Err when all the senders will be dropped.
                                        // VERIFIER_THREADS_NUM senders will be dropped when threads will finish verifying files
                                        // but this one will live as long as current thread exists so we should drop it manually
                                        drop(sender);

                                        let mut broken = Vec::new();
                                        let mut processed = 0;

                                        while let Ok((file, status)) = receiver.recv() {
                                            processed += file.size;

                                            if !status {
                                                broken.push(file);
                                            }

                                            let progress = processed as f64 / total as f64;

                                            this.update(Actions::UpdateProgress {
                                                fraction: Rc::new(progress),
                                                title: Rc::new(format!("Verifying files: {:.2}%", progress * 100.0))
                                            }).unwrap();
                                        }

                                        if !broken.is_empty() {
                                            this.update(Actions::UpdateProgress {
                                                fraction: Rc::new(0.0),
                                                title: Rc::new(String::from("Repairing files: 0%"))
                                            }).unwrap();

                                            println!("Found broken files:");

                                            for file in &broken {
                                                println!(" - {:?}", file.path);
                                            }

                                            let total = broken.len() as f64;

                                            let is_patch_applied = match Patch::try_fetch(config.patch.servers, consts::PATCH_FETCHING_TIMEOUT) {
                                                Ok(patch) => patch.is_applied(&config.game.path).unwrap_or(true),
                                                Err(_) => true
                                            };

                                            println!("Patch status: {}", is_patch_applied);

                                            fn should_ignore(path: &Path) -> bool {
                                                for part in ["UnityPlayer.dll", "xlua.dll", "crashreport.exe", "upload_crash.exe", "vulkan-1.dll"] {
                                                    if path.ends_with(part) {
                                                        return true;
                                                    }
                                                }

                                                false
                                            }

                                            for (i, file) in broken.into_iter().enumerate() {
                                                if !is_patch_applied || !should_ignore(&file.path) {
                                                    println!("Repairing: {:?}", &file.path);

                                                    if let Err(err) = file.repair(&config.game.path) {
                                                        let err: Error = err.into();

                                                        this.update(Actions::Toast(Rc::new((
                                                            String::from("Failed to repair game file"), err.to_string()
                                                        )))).unwrap();
                                                    }
                                                }

                                                let progress = i as f64 / total;

                                                this.update(Actions::UpdateProgress {
                                                    fraction: Rc::new(progress),
                                                    title: Rc::new(format!("Repairing files: {:.2}%", progress * 100.0))
                                                }).unwrap();
                                            }
                                        }

                                        this.update(Actions::HideProgressBar).unwrap();
                                    },
                                    Err(err) => {
                                        this.update(Actions::Toast(Rc::new((
                                            String::from("Failed to get integrity files"), err.to_string()
                                        )))).unwrap();

                                        this.update(Actions::HideProgressBar).unwrap();
                                    }
                                }
                            });
                        },
                        Err(err) => this.toast("Failed to load config", err)
                    }
                }

                Actions::ShowProgressBar => {
                    this.widgets.progress_bar.show();
                }

                Actions::UpdateProgress { fraction, title } => {
                    this.widgets.progress_bar.update(*fraction, Some(title.as_str()));
                }

                Actions::HideProgressBar => {
                    this.widgets.progress_bar.hide();
                }

                Actions::Toast(toast) => {
                    let (msg, err) = (toast.0.clone(), toast.1.to_string());

                    this.toast(msg, err);
                }
            }

            glib::Continue(true)
        });

        self.actions.set(Some(sender));

        self
    }

    /// Update widgets state by calling some action
    pub fn update(&self, action: Actions) -> Result<(), std::sync::mpsc::SendError<Actions>> {
        let actions = self.actions.take();
        
        let result = match &actions {
            Some(sender) => Ok(sender.send(action)?),
            None => Ok(())
        };

        self.actions.set(actions);

        result
    }

    /// Show application window
    pub fn show(&self) {
        self.widgets.window.show();
    }

    pub fn set_state(&self, state: LauncherState) {
        println!("[main] [set_state] state: {:?}", &state);

        self.widgets.progress_bar.hide();

        self.widgets.launch_game.set_tooltip_text(None);
        self.widgets.launch_game.set_sensitive(true);

        self.widgets.launch_game.add_css_class("suggested-action");
        self.widgets.launch_game.remove_css_class("warning");
        self.widgets.launch_game.remove_css_class("destructive-action");

        self.widgets.predownload_game.hide();

        match &state {
            LauncherState::Launch => {
                self.widgets.launch_game.set_label("Launch");
            }

            LauncherState::PredownloadAvailable { game, voices } => {
                self.widgets.launch_game.set_label("Launch");

                // Calculate size of the update
                let size =
                    game.size().unwrap_or((0, 0)).0 +
                    voices.iter().fold(0, |acc, voice| acc + voice.size().unwrap_or((0, 0)).0);

                // Update tooltip
                self.widgets.predownload_game.set_tooltip_text(Some(&format!("Pre-download {} update ({})", game.latest(), prettify_bytes(size))));

                // Prepare button's color
                self.widgets.predownload_game.remove_css_class("success");
                self.widgets.predownload_game.add_css_class("warning");
                self.widgets.predownload_game.set_sensitive(true);

                if let Ok(config) = config::get() {
                    if let Some(temp) = config.launcher.temp {
                        // If all the files were downloaded
                        let downloaded =
                            temp.join(game.file_name().unwrap()).exists() &&
                            voices.iter().all(|voice| temp.join(voice.file_name().unwrap()).exists());

                        if downloaded {
                            self.widgets.predownload_game.remove_css_class("warning");
                            self.widgets.predownload_game.add_css_class("success");
                            self.widgets.predownload_game.set_sensitive(false);

                            self.widgets.predownload_game.set_tooltip_text(Some(&format!("{} update is predownloaded ({})", game.latest(), prettify_bytes(size))));
                        }
                    }
                }

                self.widgets.predownload_game.show();
            }

            LauncherState::PatchAvailable(patch) => {
                match patch {
                    Patch::NotAvailable => {
                        self.widgets.launch_game.set_label("Patch not available");

                        self.widgets.launch_game.set_tooltip_text(Some("Patch servers are unavailable and launcher can't verify the game's patching status. You're allowed to run the game on your own risk"));

                        self.widgets.launch_game.remove_css_class("suggested-action");
                        self.widgets.launch_game.add_css_class("destructive-action");
                    }

                    Patch::Outdated { .. } |
                    Patch::Preparation { .. } => {
                        self.widgets.launch_game.set_label("Patch not available");
                        self.widgets.launch_game.set_sensitive(false);

                        self.widgets.launch_game.set_tooltip_text(Some("Patch is outdated or in preparation state, so unavailable for usage. Return back later to see its status"));

                        self.widgets.launch_game.remove_css_class("suggested-action");
                        self.widgets.launch_game.add_css_class("destructive-action");
                    }

                    Patch::Testing { .. } => {
                        self.widgets.launch_game.set_label("Apply test patch");

                        self.widgets.launch_game.remove_css_class("suggested-action");
                        self.widgets.launch_game.add_css_class("warning");
                    }

                    Patch::Available { .. } => {
                        self.widgets.launch_game.set_label("Apply patch");
                    }
                }
            }

            LauncherState::WineNotInstalled => {
                self.widgets.launch_game.set_label("Download wine");
            }

            LauncherState::PrefixNotExists => {
                self.widgets.launch_game.set_label("Create prefix");
            }

            LauncherState::GameUpdateAvailable(_) |
            LauncherState::VoiceUpdateAvailable(_) => {
                self.widgets.launch_game.set_label("Update");
            }

            LauncherState::GameNotInstalled(_) |
            LauncherState::VoiceNotInstalled(_) => {
                self.widgets.launch_game.set_label("Download");
            }

            LauncherState::VoiceOutdated(_) |
            LauncherState::GameOutdated(_) => {
                self.widgets.launch_game.set_label("Update");
                self.widgets.launch_game.set_tooltip_text(Some("Version is too outdated and can't be updated"));
                self.widgets.launch_game.set_sensitive(false);
            }
        }

        let mut values = self.values.take();

        values.state = state;

        self.values.set(values);
    }

    pub fn update_state(&self) -> Await<Result<LauncherState, String>> {
        self.widgets.status_page.show();
        self.widgets.launcher_content.hide();

        let (send, recv) = std::sync::mpsc::channel();

        let this = self.clone();

        glib::MainContext::default().invoke(move || {
            let (sender, receiver) = glib::MainContext::channel::<String>(glib::PRIORITY_DEFAULT);

            receiver.attach(None, clone!(@strong this.widgets.status_page as status_page => move |description| {
                status_page.set_description(Some(&description));

                glib::Continue(true)
            }));

            std::thread::spawn(move || {
                match LauncherState::get(move |status| sender.send(status.to_string()).unwrap()) {
                    Ok(state) => {
                        this.set_state(state.clone());

                        this.widgets.status_page.hide();
                        this.widgets.launcher_content.show();

                        send.send(Ok(state)).unwrap();
                    },
                    Err(err) => {
                        send.send(Err(err.to_string())).unwrap();

                        glib::MainContext::default().invoke(move || {
                            this.toast("Failed to get initial launcher state", err);
                        });
                    }
                }
            });
        });

        Await::new(move || {
            recv.recv().unwrap()
        })
    }
}

impl Toast for App {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay) {
        (self.widgets.window.clone(), self.widgets.toast_overlay.clone())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
