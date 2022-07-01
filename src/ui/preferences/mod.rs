use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::Error;

use crate::ui::get_object;

mod general_page;
mod enhanced_page;

pub mod pages {
    pub use super::general_page::Page as GeneralPage;
    pub use super::enhanced_page::Page as EnhancedPage;
}

#[derive(Clone)]
pub struct PreferencesStack {
    pub preferences: gtk::Box,
    pub preferences_go_back: gtk::Button,

    pub stack: gtk::Stack,

    pub general_page: pages::GeneralPage,
    pub enhanced_page: pages::EnhancedPage
}

impl PreferencesStack {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences.ui"));

        let result = Self {
            preferences: get_object(&builder, "preferences")?,
            preferences_go_back: get_object(&builder, "preferences_go_back")?,
            stack: get_object(&builder, "stack")?,
            general_page: pages::GeneralPage::new()?,
            enhanced_page: pages::EnhancedPage::new()?
        };

        result.stack.add_titled(&result.general_page.page, None, &pages::GeneralPage::title());
        result.stack.add_titled(&result.enhanced_page.page, None, &pages::EnhancedPage::title());

        Ok(result)
    }

    /// Update page info before opening it
    /// 
    /// Being called from the `MainApp` struct
    /// 
    /// TODO: do it asynchronously. The problem is that I somehow need to handle this function's error to display it as a toast
    pub fn update(&self) -> Result<(), Error> {
        self.general_page.update()?;
        self.enhanced_page.update()?;

        Ok(())
    }
}
