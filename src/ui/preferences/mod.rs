use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;

use std::io::Error;

use crate::ui::*;
use crate::ui::traits::prelude::*;

mod general_page;
mod enhancements_page;

pub mod pages {
    pub use super::general_page::App as GeneralPage;
    pub use super::enhancements_page::App as EnhancementsPage;
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
    pub enhancements_page: pages::EnhancementsPage
}

impl PreferencesStack {
    pub fn new(window: adw::ApplicationWindow, toast_overlay: adw::ToastOverlay) -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences.ui"));

        let result = Self {
            window: window.clone(),
            toast_overlay: toast_overlay.clone(),

            preferences: get_object(&builder, "preferences")?,
            preferences_go_back: get_object(&builder, "preferences_go_back")?,

            status_page: get_object(&builder, "status_page")?,
            flap: get_object(&builder, "flap")?,

            stack: get_object(&builder, "stack")?,
            
            general_page: pages::GeneralPage::new(window, toast_overlay)?,
            enhancements_page: pages::EnhancementsPage::new()?
        };

        result.stack.add_titled(&result.general_page.get_page(), None, &pages::GeneralPage::title());
        result.stack.add_titled(&result.enhancements_page.get_page(), None, &pages::EnhancementsPage::title());

        Ok(result)
    }

    /// Update page info before opening it
    /// 
    /// Being called from the `MainApp` struct
    pub fn update(&self) -> Result<(), Error> {
        self.status_page.set_visible(true);
        self.status_page.set_description(None);
        self.flap.set_visible(false);

        self.general_page.prepare(&self.status_page)?;
        self.enhancements_page.prepare(&self.status_page)?;

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

unsafe impl Send for PreferencesStack {}
unsafe impl Sync for PreferencesStack {}
