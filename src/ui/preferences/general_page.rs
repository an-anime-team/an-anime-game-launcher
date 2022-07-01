use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use crate::ui::get_object;

#[derive(Clone)]
pub struct Page {
    pub page: adw::PreferencesPage,
    pub game_version: gtk::Label,
    pub patch_version: gtk::Label
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences_general.ui"));

        Ok(Self {
            page: get_object(&builder, "general_page")?,
            game_version: get_object(&builder, "game_version")?,
            patch_version: get_object(&builder, "patch_version")?
        })
    }

    pub fn title() -> String {
        String::from("General")
    }
}
