use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::Error;

use crate::ui::get_object;
use crate::lib::config;

#[derive(Clone)]
pub struct Page {
    pub page: adw::PreferencesPage,

    pub sync_combo: adw::ComboRow,
    pub fsr_combo: adw::ComboRow,
    pub fsr_switcher: gtk::Switch,
    pub wine_lang: adw::ComboRow,

    pub hud_combo: adw::ComboRow,
    pub gamemode_switcher: gtk::Switch
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences_enhanced.ui"));

        let result = Self {
            page: get_object(&builder, "enhanced_page")?,

            sync_combo: get_object(&builder, "sync_combo")?,
            fsr_combo: get_object(&builder, "fsr_combo")?,
            fsr_switcher: get_object(&builder, "fsr_switcher")?,

            hud_combo: get_object(&builder, "hud_combo")?,
            gamemode_switcher: get_object(&builder, "gamemode_switcher")?,
            wine_lang: get_object(&builder, "wine_lang")?
        };

        // Wine sync selection
        result.sync_combo.connect_selected_notify(|hud| {
            if let Ok(mut config) = config::get() {
                // TODO: show toast
                config.game.wine.sync = config::WineSync::try_from(hud.selected()).unwrap();

                config::update(config).unwrap();
            }
        });

        // FSR strength selection
        result.fsr_combo.connect_selected_notify(|hud| {
            if let Ok(mut config) = config::get() {
                // TODO: show toast

                // Ultra Quality = 5
                // Quality       = 4
                // Balanced      = 3
                // Performance   = 2
                // 
                // Source: Bottles (https://github.com/bottlesdevs/Bottles/blob/22fa3573a13f4e9b9c429e4cdfe4ca29787a2832/src/ui/details-preferences.ui#L88)
                config.game.enhancements.fsr.strength = 5 - hud.selected();

                config::update(config).unwrap();
            }
        });

        // FSR switching
        result.fsr_switcher.connect_state_notify(|switcher| {
            if let Ok(mut config) = config::get() {
                // TODO: show toast
                config.game.enhancements.fsr.enabled = switcher.state();

                config::update(config).unwrap();
            }
        });

        // Wine language selection
        result.wine_lang.connect_selected_notify(|hud| {
            if let Ok(mut config) = config::get() {
                // TODO: show toast
                config.game.wine.language = config::WineLang::try_from(hud.selected()).unwrap();

                config::update(config).unwrap();
            }
        });

        // HUD selection
        result.hud_combo.connect_selected_notify(|hud| {
            if let Ok(mut config) = config::get() {
                // TODO: show toast
                config.game.enhancements.hud = config::HUD::try_from(hud.selected()).unwrap();

                config::update(config).unwrap();
            }
        });
        
        // Gamemode switching
        result.gamemode_switcher.connect_state_notify(|switcher| {
            if let Ok(mut config) = config::get() {
                // TODO: show toast
                config.game.enhancements.gamemode = switcher.state();

                config::update(config).unwrap();
            }
        });

        Ok(result)
    }

    pub fn title() -> String {
        String::from("Enhanced")
    }

    /// This method is being called by the `PreferencesStack::update`
    pub fn update(&self, status_page: &adw::StatusPage) -> Result<(), Error> {
        let config = config::get()?;

        status_page.set_description(Some("Loading preferences..."));

        // Update Wine sync
        self.sync_combo.set_selected(config.game.wine.sync.into());

        // FSR strength selection
        self.fsr_combo.set_selected(5 - config.game.enhancements.fsr.strength);

        // FSR switching
        self.fsr_switcher.set_state(config.game.enhancements.fsr.enabled);

        // Update wine language
        self.wine_lang.set_selected(config.game.wine.language.into());

        // Update HUD
        self.hud_combo.set_selected(config.game.enhancements.hud.into());

        // Gamemode switching
        self.fsr_switcher.set_state(config.game.enhancements.gamemode);

        Ok(())
    }
}
