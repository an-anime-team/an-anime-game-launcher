use crate::ui::*;

#[derive(Clone)]
pub struct Page {
    pub page: gtk::Box,
    pub continue_button: gtk::Button,
    pub advanced_button: gtk::Button
}

impl Page {
    pub fn new() -> anyhow::Result<Self> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run/welcome.ui");

        Ok(Self {
            page: get_object(&builder, "page")?,
            continue_button: get_object(&builder, "continue_button")?,
            advanced_button: get_object(&builder, "advanced_button")?
        })
    }
}
