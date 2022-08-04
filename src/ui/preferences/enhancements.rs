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

    pub hud_combo: adw::ComboRow,
    pub fsr_combo: adw::ComboRow,
    pub fsr_switcher: gtk::Switch,

    pub gamemode_row: adw::ActionRow,
    pub gamemode_switcher: gtk::Switch,

    pub gamescope_row: adw::ActionRow,
    pub gamescope_settings: gtk::Button,
    pub gamescope_switcher: gtk::Switch,

    pub gamescope_app: GamescopeApp
}

impl AppWidgets {
    fn try_get(window: &adw::ApplicationWindow) -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/preferences/enhancements.ui");

        let result = Self {
            page: get_object(&builder, "page")?,

            sync_combo: get_object(&builder, "sync_combo")?,
            wine_lang: get_object(&builder, "wine_lang")?,

            hud_combo: get_object(&builder, "hud_combo")?,
            fsr_combo: get_object(&builder, "fsr_combo")?,
            fsr_switcher: get_object(&builder, "fsr_switcher")?,

            gamemode_row: get_object(&builder, "gamemode_row")?,
            gamemode_switcher: get_object(&builder, "gamemode_switcher")?,

            gamescope_row: get_object(&builder, "gamescope_row")?,
            gamescope_settings: get_object(&builder, "gamescope_settings")?,
            gamescope_switcher: get_object(&builder, "gamescope_switcher")?,

            gamescope_app: GamescopeApp::new(window)?
        };

        // Set availale wine languages
        let model = gtk::StringList::new(&[]);

        for lang in WineLang::list() {
            let lang: String = lang.into();

            model.append(&lang);
        }

        result.wine_lang.set_model(Some(&model));

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

        // Update HUD
        self.widgets.hud_combo.set_selected(config.game.enhancements.hud.into());

        // FSR strength selection
        self.widgets.fsr_combo.set_selected(5 - config.game.enhancements.fsr.strength as u32);

        // FSR switching
        self.widgets.fsr_switcher.set_state(config.game.enhancements.fsr.enabled);

        // Gamemode switching
        self.widgets.gamemode_switcher.set_state(config.game.enhancements.gamemode);

        // Prepare gamescope settings app
        self.widgets.gamescope_app.prepare(status_page)?;

        Ok(())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
