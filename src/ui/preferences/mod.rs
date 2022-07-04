use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk4::glib;

use std::io::Error;

use crate::ui::get_object;
use crate::ui::ToastError;

mod general_page;
mod enhanced_page;

pub mod pages {
    pub use super::general_page::App as GeneralPage;
    pub use super::enhanced_page::App as EnhancedPage;
}

#[derive(Clone, glib::Downgrade)]
pub struct PreferencesStack {
    pub window: adw::ApplicationWindow,
    pub toast_overlay: adw::ToastOverlay,

    pub preferences: gtk::Box,
    pub preferences_go_back: gtk::Button,

    pub status_page: adw::StatusPage,
    pub flap: adw::Flap,

    pub stack: gtk::Stack,

    pub general_page: pages::GeneralPage,
    pub enhanced_page: pages::EnhancedPage
}

impl PreferencesStack {
    pub fn new(window: adw::ApplicationWindow, toast_overlay: adw::ToastOverlay) -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences.ui"));

        let result = Self {
            window,
            toast_overlay,

            preferences: get_object(&builder, "preferences")?,
            preferences_go_back: get_object(&builder, "preferences_go_back")?,

            status_page: get_object(&builder, "status_page")?,
            flap: get_object(&builder, "flap")?,

            stack: get_object(&builder, "stack")?,
            
            general_page: pages::GeneralPage::new()?,
            enhanced_page: pages::EnhancedPage::new()?
        };

        result.stack.add_titled(&result.general_page.get_page(), None, &pages::GeneralPage::title());
        result.stack.add_titled(&result.enhanced_page.get_page(), None, &pages::EnhancedPage::title());

        Ok(result)
    }

    /// Update page info before opening it
    /// 
    /// Being called from the `MainApp` struct
    /// 
    /// TODO: do it asynchronously. The problem is that I somehow need to handle this function's error to display it as a toast
    pub fn update(&self) -> Result<(), Error> {
        self.status_page.set_visible(true);
        self.status_page.set_description(None);
        self.flap.set_visible(false);

        self.general_page.prepare(&self.status_page)?;
        self.enhanced_page.prepare(&self.status_page)?;

        self.status_page.set_visible(false);
        self.flap.set_visible(true);

        Ok(())
    }
}

impl ToastError for PreferencesStack {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay) {
        (self.window.clone(), self.toast_overlay.clone())
    }
}
