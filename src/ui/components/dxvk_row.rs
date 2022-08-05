use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use crate::lib::dxvk::Version;
use crate::ui::traits::download_component::*;

#[derive(Debug, Clone)]
pub struct DxvkRow {
    pub version: Version,

    pub row: adw::ActionRow,
    pub button: gtk::Button,
    pub apply_button: gtk::Button,
    pub progress_bar: gtk::ProgressBar
}

impl DxvkRow {
    pub fn new(version: Version) -> Self {
        let row = adw::ActionRow::new();
        let button = gtk::Button::new();
        let apply_button = gtk::Button::new();

        row.set_title(&version.version);
        row.set_visible(version.recommended);

        apply_button.set_icon_name("view-refresh-symbolic");
        apply_button.set_valign(gtk::Align::Center);
        apply_button.add_css_class("flat");
        apply_button.set_tooltip_text(Some("Apply"));
        apply_button.hide();

        row.add_suffix(&apply_button);

        button.set_icon_name("document-save-symbolic");
        button.set_valign(gtk::Align::Center);
        button.add_css_class("flat");

        row.add_suffix(&button);

        let progress_bar = gtk::ProgressBar::new();

        progress_bar.set_text(Some("Downloading: 0%"));
        progress_bar.set_show_text(true);

        progress_bar.set_width_request(200);
        progress_bar.set_valign(gtk::Align::Center);
        progress_bar.hide();

        row.add_suffix(&progress_bar);

        Self {
            version,
            row,
            button,
            apply_button,
            progress_bar
        }
    }

    pub fn update_state<T: ToString>(&self, dxvks_folder: T) {
        if self.is_downloaded(dxvks_folder) {
            self.button.set_icon_name("user-trash-symbolic");

            self.apply_button.show();
        }

        else {
            self.button.set_icon_name("document-save-symbolic");

            self.apply_button.hide();
        }
    }

    pub fn apply<T: ToString>(&self, dxvks_folder: T, prefix_path: T) -> std::io::Result<std::process::Output> {
        self.button.set_sensitive(false);
        self.apply_button.set_sensitive(false);

        let result = self.version.apply(dxvks_folder, prefix_path);

        self.button.set_sensitive(true);
        self.apply_button.set_sensitive(true);

        result
    }
}

impl DownloadComponent for DxvkRow {
    fn get_component_path<T: ToString>(&self, installation_path: T) -> String {
        format!("{}/{}", installation_path.to_string(), self.version.name)
    }

    fn get_downloading_widgets(&self) -> (gtk::ProgressBar, gtk::Button) {
        (self.progress_bar.clone(), self.button.clone())
    }

    fn get_download_uri(&self) -> String {
        self.version.uri.clone()
    }
}

unsafe impl Send for DxvkRow {}
unsafe impl Sync for DxvkRow {}
