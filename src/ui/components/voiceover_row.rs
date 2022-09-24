use gtk::prelude::*;
use adw::prelude::*;

use std::path::PathBuf;

use anime_game_core::genshin::voice_data::package::VoicePackage;

#[derive(Debug, Clone)]
pub struct VoiceoverRow {
    pub package: VoicePackage,

    pub row: adw::ActionRow,
    pub button: gtk::Button
}

impl VoiceoverRow {
    pub fn new(package: VoicePackage) -> Self {
        let row = adw::ActionRow::new();
        let button = gtk::Button::new();

        row.set_title(package.locale().to_name());

        button.set_icon_name("document-save-symbolic");
        button.set_valign(gtk::Align::Center);
        button.add_css_class("flat");

        row.add_suffix(&button);

        Self {
            package,
            row,
            button
        }
    }

    pub fn update_state<T: Into<PathBuf>>(&self, game_path: T) {
        if self.is_downloaded(game_path) {
            self.button.set_icon_name("user-trash-symbolic");
        }

        else {
            self.button.set_icon_name("document-save-symbolic");
        }
    }

    pub fn is_downloaded<T: Into<PathBuf>>(&self, game_path: T) -> bool {
        self.package.is_installed_in(game_path)
    }
}

unsafe impl Send for VoiceoverRow {}
unsafe impl Sync for VoiceoverRow {}
