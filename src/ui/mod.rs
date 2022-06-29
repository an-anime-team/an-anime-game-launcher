use gtk4::{self as gtk, prelude::*};

mod main;
mod preferences;

pub use main::App as MainApp;

/// This function loads object from builder or panics if it doesn't exist
pub fn get_object<T: IsA<gtk::glib::Object>>(builder: &gtk::Builder, name: &str) -> Result<T, String> {
    match builder.object::<T>(name) {
        Some(object) => Ok(object),
        None => Err(format!("Failed to parse object '{}'", name))
    }
}
