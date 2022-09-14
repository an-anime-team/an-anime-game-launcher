use gtk4 as gtk;
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::glib::clone;

use crate::lib;
use crate::lib::config;
use crate::lib::config::prelude::*;

use crate::ui::*;

use super::gamescope::App as GamescopeApp;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub page: adw::PreferencesPage,

    pub sync_combo: adw::ComboRow,
    pub wine_lang: adw::ComboRow,
    pub borderless: gtk::Switch,
    pub virtual_desktop_row: adw::ComboRow,
    pub virtual_desktop: gtk::Switch,

    pub hud_combo: adw::ComboRow,
    pub fsr_combo: adw::ComboRow,
    pub fsr_switcher: gtk::Switch,

    pub gamemode_row: adw::ActionRow,
    pub gamemode_switcher: gtk::Switch,

    pub gamescope_row: adw::ActionRow,
    pub gamescope_settings: gtk::Button,
    pub gamescope_switcher: gtk::Switch,

    pub gamescope_app: GamescopeApp,

    pub fps_unlocker_combo: adw::ComboRow,
    pub fps_unlocker_switcher: gtk::Switch,
    pub fps_unlocker_power_saving_switcher: gtk::Switch,
    pub fps_unlocker_fullscreen_switcher: gtk::Switch,
    pub fps_unlocker_priority_combo: adw::ComboRow
}

impl AppWidgets {
    fn try_get(window: &adw::ApplicationWindow) -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/preferences/enhancements.ui");

        let result = Self {
            page: get_object(&builder, "page")?,

            sync_combo: get_object(&builder, "sync_combo")?,
            wine_lang: get_object(&builder, "wine_lang")?,
            borderless: get_object(&builder, "borderless")?,
            virtual_desktop_row: get_object(&builder, "virtual_desktop_row")?,
            virtual_desktop: get_object(&builder, "virtual_desktop")?,

            hud_combo: get_object(&builder, "hud_combo")?,
            fsr_combo: get_object(&builder, "fsr_combo")?,
            fsr_switcher: get_object(&builder, "fsr_switcher")?,

            gamemode_row: get_object(&builder, "gamemode_row")?,
            gamemode_switcher: get_object(&builder, "gamemode_switcher")?,

            gamescope_row: get_object(&builder, "gamescope_row")?,
            gamescope_settings: get_object(&builder, "gamescope_settings")?,
            gamescope_switcher: get_object(&builder, "gamescope_switcher")?,

            gamescope_app: GamescopeApp::new(window)?,

            fps_unlocker_combo: get_object(&builder, "fps_unlocker_combo")?,
            fps_unlocker_switcher: get_object(&builder, "fps_unlocker_switcher")?,
            fps_unlocker_power_saving_switcher: get_object(&builder, "fps_unlocker_power_saving_switcher")?,
            fps_unlocker_fullscreen_switcher: get_object(&builder, "fps_unlocker_fullscreen_switcher")?,
            fps_unlocker_priority_combo: get_object(&builder, "fps_unlocker_priority_combo")?
        };

        // Set availale wine languages
        result.wine_lang.set_model(Some(&WineLang::get_model()));

        // Set availale virtual desktop resolutions
        result.virtual_desktop_row.set_model(Some(&Resolution::get_model()));

        // Set availale fps unlocker limits
        result.fps_unlocker_combo.set_model(Some(&Fps::get_model()));

        // Disable gamemode row if it's not available
        if !lib::is_available("gamemoderun") {
            result.gamemode_row.set_sensitive(false);
            result.gamemode_row.set_tooltip_text(Some("Gamemode is not installed"));
        }

        // Disable gamescope row if it's not available
        if !lib::is_available("gamescope") {
            result.gamescope_row.set_sensitive(false);
            result.gamescope_row.set_tooltip_text(Some("Gamescope is not installed"));
        }

        Ok(result)
    }
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
    widgets: AppWidgets
}

impl App {
    /// Create new application
    pub fn new(window: &adw::ApplicationWindow) -> Result<Self, String> {
        let result = Self {
            widgets: AppWidgets::try_get(window)?
        }.init_events();

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Wine sync selection
        self.widgets.sync_combo.connect_selected_notify(move |row| {
            if let Ok(mut config) = config::get() {
                config.game.wine.sync = WineSync::try_from(row.selected()).unwrap();

                config::update(config);
            }
        });

        // Wine language selection
        self.widgets.wine_lang.connect_selected_notify(move |row| {
            if let Ok(mut config) = config::get() {
                config.game.wine.language = WineLang::list()[row.selected() as usize];

                config::update(config);
            }
        });

        // Borderless switching
        self.widgets.borderless.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.wine.borderless = switch.state();

                config::update(config);
            }
        });

        // Virtual desktop resolution selection
        self.widgets.virtual_desktop_row.connect_selected_notify(move |row| {
            if let Ok(mut config) = config::get() {
                let resolutions = Resolution::list();

                if row.selected() > 0 {
                    let (w, h) = resolutions[row.selected() as usize - 1].get_pair();

                    config.game.wine.virtual_desktop.width = w;
                    config.game.wine.virtual_desktop.height = h;

                    config::update(config);
                }
            }
        });

        // Virtual desktop switching
        self.widgets.virtual_desktop.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.wine.virtual_desktop.enabled = switch.state();

                config::update(config);
            }
        });

        // HUD selection
        self.widgets.hud_combo.connect_selected_notify(move |row| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.hud = HUD::try_from(row.selected()).unwrap();

                config::update(config);
            }
        });

        // FSR strength selection
        // 
        // Ultra Quality = 5
        // Quality       = 4
        // Balanced      = 3
        // Performance   = 2
        // 
        // Source: Bottles (https://github.com/bottlesdevs/Bottles/blob/22fa3573a13f4e9b9c429e4cdfe4ca29787a2832/src/ui/details-preferences.ui#L88)
        self.widgets.fsr_combo.connect_selected_notify(move |row| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.fsr.strength = 5 - row.selected() as u64;

                config::update(config);
            }
        });

        // FSR switching
        self.widgets.fsr_switcher.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.fsr.enabled = switch.state();

                config::update(config);
            }
        });
        
        // Gamemode switching
        self.widgets.gamemode_switcher.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamemode = switch.state();

                config::update(config);
            }
        });

        // Gamescope settings app
        self.widgets.gamescope_settings.connect_clicked(clone!(@weak self as this => move |_| {
            this.widgets.gamescope_app.show();
        }));

        // Gamescope swithing
        self.widgets.gamescope_switcher.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.enabled = switch.state();

                config::update(config);
            }
        });

        // FPS unlocker swithing
        self.widgets.fps_unlocker_switcher.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.fps_unlocker.enabled = switch.state();

                config::update(config);
            }
        });

        // FPS unlocker -> fps limit combo
        self.widgets.fps_unlocker_combo.connect_selected_notify(move |row| {
            if let Ok(mut config) = config::get() {
                if row.selected() > 0 {
                    config.game.enhancements.fps_unlocker.config.fps = Fps::list()[row.selected() as usize - 1].to_num();

                    config::update(config);
                }
            }
        });

        // FPS unlocker -> power saving swithing
        self.widgets.fps_unlocker_power_saving_switcher.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.fps_unlocker.config.power_saving = switch.state();

                config::update(config);
            }
        });

        // FPS unlocker -> fullscreen swithing
        self.widgets.fps_unlocker_fullscreen_switcher.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.fps_unlocker.config.fullscreen = switch.state();

                config::update(config);
            }
        });

        // FPS unlocker -> priority combo
        self.widgets.fps_unlocker_priority_combo.connect_selected_notify(move |row| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.fps_unlocker.config.priority = row.selected() as u64;

                config::update(config);
            }
        });

        self
    }

    pub fn title() -> String {
        String::from("Enhancements")
    }

    pub fn get_page(&self) -> adw::PreferencesPage {
        self.widgets.page.clone()
    }

    /// This method is being called by the `PreferencesStack::update`
    pub fn prepare(&self, status_page: &adw::StatusPage) -> std::io::Result<()> {
        let config = config::get()?;

        status_page.set_description(Some("Loading enhancements..."));

        // Update Wine sync
        self.widgets.sync_combo.set_selected(config.game.wine.sync.into());

        // Update wine language
        self.widgets.wine_lang.set_selected(config.game.wine.language.into());

        // Update borderless
        self.widgets.borderless.set_state(config.game.wine.borderless);
        
        // Update virtual desktop
        self.widgets.virtual_desktop.set_state(config.game.wine.virtual_desktop.enabled);

        let resolution = Resolution::from_pair(
            config.game.wine.virtual_desktop.width,
            config.game.wine.virtual_desktop.height
        );

        if let Resolution::Custom(_, _) = resolution {
            self.widgets.virtual_desktop_row.set_selected(0);
        }

        else {
            for (i, res) in Resolution::list().into_iter().enumerate() {
                if res == resolution {
                    self.widgets.virtual_desktop_row.set_selected(i as u32 + 1);
                }
            }
        }

        // Update HUD
        self.widgets.hud_combo.set_selected(config.game.enhancements.hud.into());

        // FSR strength selection
        self.widgets.fsr_combo.set_selected(5 - config.game.enhancements.fsr.strength as u32);

        // FSR switching
        self.widgets.fsr_switcher.set_state(config.game.enhancements.fsr.enabled);

        // Gamemode switching
        self.widgets.gamemode_switcher.set_state(config.game.enhancements.gamemode);

        // Switch gamescope option
        self.widgets.gamescope_switcher.set_state(config.game.enhancements.gamescope.enabled);

        // Switch FPS unlocker
        self.widgets.fps_unlocker_switcher.set_state(config.game.enhancements.fps_unlocker.enabled);

        // Select FPS limit
        let fps = Fps::from_num(config.game.enhancements.fps_unlocker.config.fps);

        if let Fps::Custom(_) = fps {
            self.widgets.fps_unlocker_combo.set_selected(0);
        }

        else {
            for (i, value) in Fps::list().into_iter().enumerate() {
                if value == fps {
                    self.widgets.fps_unlocker_combo.set_selected(i as u32 + 1);
                }
            }
        }

        // Switch FPS unlocker -> power saving
        self.widgets.fps_unlocker_power_saving_switcher.set_state(config.game.enhancements.fps_unlocker.config.power_saving);

        // Switch FPS unlocker -> fullscreen
        self.widgets.fps_unlocker_fullscreen_switcher.set_state(config.game.enhancements.fps_unlocker.config.fullscreen);

        // Switch FPS unlocker -> priority
        self.widgets.fps_unlocker_priority_combo.set_selected(config.game.enhancements.fps_unlocker.config.priority as u32);

        // Prepare gamescope settings app
        self.widgets.gamescope_app.prepare(status_page)?;

        Ok(())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
