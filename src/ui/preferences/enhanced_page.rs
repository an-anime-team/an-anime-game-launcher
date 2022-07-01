use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::Error;

use crate::ui::get_object;
use crate::lib::config;

#[derive(Clone)]
pub struct Page {
    pub page: adw::PreferencesPage,

    pub hud_combo: adw::ComboRow,
    pub sync_combo: adw::ComboRow,
    pub fsr_combo: adw::ComboRow
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences_enhanced.ui"));

        let result = Self {
            page: get_object(&builder, "enhanced_page")?,

            hud_combo: get_object(&builder, "hud_combo")?,
            sync_combo: get_object(&builder, "sync_combo")?,
            fsr_combo: get_object(&builder, "fsr_combo")?
        };

        // Wine HUD selection
        result.hud_combo.connect_selected_notify(|hud| {
            if let Ok(mut config) = config::get() {
                // TODO: show toast
                config.game.enhancements.hud = config::WineHUD::try_from(hud.selected()).unwrap();

                config::update(config).unwrap();
            }
        });

        Ok(result)
    }

    pub fn title() -> String {
        String::from("Enhanced")
    }

    /// This method is being called by the `PreferencesStack::update`
    pub fn update(&self) -> Result<(), Error> {
        let config = config::get()?;

        // Update Wine HUD
        self.hud_combo.set_selected(config.game.enhancements.hud.into());

        Ok(())
    }
}
