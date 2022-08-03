use gtk4 as gtk;

use crate::ui::*;

#[derive(Clone)]
pub struct Page {
    pub page: gtk::Box,
    pub continue_button: gtk::Button,
    pub exit_button: gtk::Button
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run/tos_warning.ui");

        Ok(Self {
            page: get_object(&builder, "page")?,
            continue_button: get_object(&builder, "continue_button")?,
            exit_button: get_object(&builder, "exit_button")?
        })
    }
}
