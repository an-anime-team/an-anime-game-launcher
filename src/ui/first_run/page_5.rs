use gtk4 as gtk;
use libadwaita as adw;

use crate::ui::*;
use crate::ui::components::progress_bar::ProgressBar;

#[derive(Clone)]
pub struct Page {
    pub page: gtk::Box,

    pub wine_version: adw::ComboRow,
    pub dxvk_version: adw::ComboRow,

    pub download_button: gtk::Button,
    pub exit_button: gtk::Button,

    pub progress_bar: ProgressBar
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/first_run/page_5.ui"));

        Ok(Self {
            page: get_object(&builder, "page")?,
            
            wine_version: get_object(&builder, "wine_version")?,
            dxvk_version: get_object(&builder, "dxvk_version")?,

            download_button: get_object(&builder, "download_button")?,
            exit_button: get_object(&builder, "exit_button")?,

            progress_bar: ProgressBar::new(
                get_object(&builder, "progress_bar")?,
                get_object(&builder, "buttons_group")?,
                get_object(&builder, "progress_bar_group")?
            )
        })
    }
}
