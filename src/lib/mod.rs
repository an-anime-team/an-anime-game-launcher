use gtk4::{self as gtk, prelude::*};

/// This function loads object from builder or panics if it doesn't exist
pub fn get_object<T: IsA<gtk::glib::Object>>(builder: &gtk::Builder, name: &str) -> T {
    builder.object::<T>(name).unwrap()
}
