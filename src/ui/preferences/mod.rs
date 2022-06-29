use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use crate::ui::get_object;

mod general_page;
mod enhanced_page;

pub mod pages {
    pub use super::general_page::Page as GeneralPage;
    pub use super::enhanced_page::Page as EnhancedPage;
}

pub struct PreferencesStack {
    pub preferences: gtk::Box,
    pub preferences_go_back: gtk::Button,
    pub stack: gtk::Stack
}

impl PreferencesStack {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences.ui"));

        let result = Self {
            preferences: get_object(&builder, "preferences")?,
            preferences_go_back: get_object(&builder, "preferences_go_back")?,
            stack: get_object(&builder, "stack")?
        };

        result.stack.add_titled(&pages::GeneralPage::get()?, None, &pages::GeneralPage::title());
        result.stack.add_titled(&pages::EnhancedPage::get()?, None, &pages::EnhancedPage::title());

        Ok(result)
    }
}
