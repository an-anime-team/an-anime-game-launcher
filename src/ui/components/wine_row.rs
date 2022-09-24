use gtk::prelude::*;
use adw::prelude::*;

use crate::lib::wine::Version;
use crate::ui::traits::download_component::*;

#[derive(Debug, Clone)]
pub struct WineRow {
    pub version: Version,

    pub row: adw::ActionRow,
    pub button: gtk::Button,
    pub progress_bar: gtk::ProgressBar
}

impl WineRow {
    pub fn new(version: Version) -> Self {
        let row = adw::ActionRow::new();
        let button = gtk::Button::new();

        row.set_title(&version.title);
        row.set_visible(version.recommended);

        button.set_icon_name("document-save-symbolic");
        button.set_valign(gtk::Align::Center);
        button.add_css_class("flat");

        row.add_suffix(&button);

        let progress_bar = gtk::ProgressBar::new();

        progress_bar.set_text(Some("Downloading: 0%"));
        progress_bar.set_show_text(true);

        progress_bar.set_width_request(200);
        progress_bar.set_valign(gtk::Align::Center);
        progress_bar.set_visible(false);

        row.add_suffix(&progress_bar);

        Self {
            version,
            row,
            button,
            progress_bar
        }
    }

    pub fn update_state<T: ToString>(&self, runners_folder: T) {
        if self.is_downloaded(runners_folder) {
            self.button.set_icon_name("user-trash-symbolic");
        }

        else {
            self.button.set_icon_name("document-save-symbolic");
        }
    }
}

impl DownloadComponent for WineRow {
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

unsafe impl Send for WineRow {}
unsafe impl Sync for WineRow {}
