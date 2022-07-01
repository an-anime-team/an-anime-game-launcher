use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use crate::ui::get_object;

#[derive(Clone)]
pub struct Page {
    pub page: adw::PreferencesPage
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences_enhanced.ui"));

        Ok(Self {
            page: get_object(&builder, "enhanced_page")?
        })
    }

    pub fn title() -> String {
        String::from("Enhanced")
    }
}
