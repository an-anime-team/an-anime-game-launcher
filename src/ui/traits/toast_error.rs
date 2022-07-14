use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use crate::ui::add_action;

pub trait ToastError {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay);

    /// Show toast with `toast` title and `See message` button
    /// 
    /// This button will show message dialog with error message
    fn toast_error<T: ToString, F: std::fmt::Display + 'static>(&self, toast: T, err: F) {
        let toast = adw::Toast::new(toast.to_string().as_str());

        toast.set_button_label(Some("See message"));
        toast.set_action_name(Some("see-message.see-message"));
        toast.set_timeout(0);

        let (window, toast_overlay) = self.get_toast_widgets();

        // Show error message in a dialog window
        add_action(&toast_overlay, "see-message", move || {
            let dialog = gtk::MessageDialog::new(
                Some(&window),
                gtk::DialogFlags::all(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Close,
                &format!("{}", err)
            );

            dialog.connect_response(move |dialog, _| {
                dialog.close();
            });

            dialog.show();
        });

        toast_overlay.add_toast(&toast);
    }
}
