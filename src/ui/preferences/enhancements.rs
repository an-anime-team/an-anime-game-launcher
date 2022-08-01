use gtk4 as gtk;
use libadwaita::{self as adw, prelude::*};

use gtk4::glib;
use gtk4::glib::clone;

use std::rc::Rc;
use std::cell::Cell;
use std::io::Error;

use crate::ui::get_object;
use crate::lib::config;

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
    pub gamemode_switcher: gtk::Switch
}

impl AppWidgets {
    fn try_get() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences/enhancements.ui"));

        let result = Self {
            page: get_object(&builder, "page")?,

            sync_combo: get_object(&builder, "sync_combo")?,
            wine_lang: get_object(&builder, "wine_lang")?,

            hud_combo: get_object(&builder, "hud_combo")?,
            fsr_combo: get_object(&builder, "fsr_combo")?,
            fsr_switcher: get_object(&builder, "fsr_switcher")?,
            gamemode_switcher: get_object(&builder, "gamemode_switcher")?
        };

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug)]
pub enum Actions<T> {
    OptionSelection(fn(crate::lib::config::Config, T) -> crate::lib::config::Config, T)
}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default, glib::Downgrade)]
pub struct Values;

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
    values: Rc<Cell<Values>>
}

impl App {
    /// Create new application
    pub fn new() -> Result<Self, String> {
        let result = Self {
            widgets: AppWidgets::try_get()?,
            values: Default::default()
        }.init_events();

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Wine sync selection
        self.widgets.sync_combo.connect_selected_notify(clone!(@weak self as this => move |hud| {
            this.update(Actions::OptionSelection(|mut config, value| {
                config.game.wine.sync = value; config
            }, config::WineSync::try_from(hud.selected()).unwrap()));
        }));

        // Wine language selection
        self.widgets.wine_lang.connect_selected_notify(clone!(@weak self as this => move |hud| {
            this.update(Actions::OptionSelection(|mut config, value| {
                config.game.wine.language = value; config
            }, config::WineLang::try_from(hud.selected()).unwrap()));
        }));

        // HUD selection
        self.widgets.hud_combo.connect_selected_notify(clone!(@weak self as this => move |hud| {
            this.update(Actions::OptionSelection(|mut config, value| {
                config.game.enhancements.hud = value; config
            }, config::HUD::try_from(hud.selected()).unwrap()));
        }));

        // FSR strength selection
        // 
        // Ultra Quality = 5
        // Quality       = 4
        // Balanced      = 3
        // Performance   = 2
        // 
        // Source: Bottles (https://github.com/bottlesdevs/Bottles/blob/22fa3573a13f4e9b9c429e4cdfe4ca29787a2832/src/ui/details-preferences.ui#L88)
        self.widgets.fsr_combo.connect_selected_notify(clone!(@weak self as this => move |hud| {
            this.update(Actions::OptionSelection(|mut config, value| {
                config.game.enhancements.fsr.strength = value; config
            }, 5 - hud.selected()));
        }));

        // FSR switching
        self.widgets.fsr_switcher.connect_state_notify(clone!(@weak self as this => move |switcher| {
            this.update(Actions::OptionSelection(|mut config, value| {
                config.game.enhancements.fsr.enabled = value; config
            }, switcher.state()));
        }));
        
        // Gamemode switching
        self.widgets.gamemode_switcher.connect_state_notify(clone!(@weak self as this => move |switcher| {
            this.update(Actions::OptionSelection(|mut config, value| {
                config.game.enhancements.gamemode = value; config
            }, switcher.state()));
        }));

        self
    }

    /// Update widgets state by calling some action
    pub fn update<T>(&self, action: Actions<T>) {
        let values = self.values.take();

        match action {
            Actions::OptionSelection(update, value) => {
                if let Ok(config) = config::get() {
                    config::update((update)(config, value));
                }
            }
        }

        self.values.set(values);
    }

    pub fn title() -> String {
        String::from("Enhancements")
    }

    pub fn get_page(&self) -> adw::PreferencesPage {
        self.widgets.page.clone()
    }

    /// This method is being called by the `PreferencesStack::update`
    pub fn prepare(&self, status_page: &adw::StatusPage) -> Result<(), Error> {
        let config = config::get()?;

        status_page.set_description(Some("Loading preferences..."));

        // Update Wine sync
        self.widgets.sync_combo.set_selected(config.game.wine.sync.into());

        // Update wine language
        self.widgets.wine_lang.set_selected(config.game.wine.language.into());

        // Update HUD
        self.widgets.hud_combo.set_selected(config.game.enhancements.hud.into());

        // FSR strength selection
        self.widgets.fsr_combo.set_selected(5 - config.game.enhancements.fsr.strength);

        // FSR switching
        self.widgets.fsr_switcher.set_state(config.game.enhancements.fsr.enabled);

        // Gamemode switching
        self.widgets.gamemode_switcher.set_state(config.game.enhancements.gamemode);

        Ok(())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
