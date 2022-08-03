use gtk4 as gtk;

use crate::ui::*;

#[derive(Clone)]
pub struct Page {
    pub page: gtk::Box,
    pub restart_button: gtk::Button,
    pub exit_button: gtk::Button
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run/finish.ui");

        Ok(Self {
            page: get_object(&builder, "page")?,
            restart_button: get_object(&builder, "restart_button")?,
            exit_button: get_object(&builder, "exit_button")?
        })
    }
}
