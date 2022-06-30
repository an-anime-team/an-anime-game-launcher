use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

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

/// Add action to widget
/// 
/// All the actions needs to be in some group. This function creates new group with the name of the action.
/// This means that to add action to some widget you need to speify `name.name` as its name
/// 
/// ## Example:
/// 
/// ```
/// let toast = libadwaita::Toast::new("Example toast");
/// 
/// toast.set_button_label(Some("Example button"));
/// toast.set_action_name(Some("example-button.example-button"));
/// 
/// add_action(&toast, "example-button", || {
///     println!("Hello, World!");
/// });
/// ```
pub fn add_action<T: IsA<gtk::Widget>, F: Fn() + 'static>(obj: &T, name: &str, closure: F) {
    let action_group = adw::gio::SimpleActionGroup::new();
    let action = adw::gio::SimpleAction::new(name, None);

    obj.insert_action_group(name, Some(&action_group));

    action.connect_activate(move |_, _| {
        closure();
    });
    
    action_group.add_action(&action);
}
