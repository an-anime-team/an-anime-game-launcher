use gtk::prelude::*;
use adw::prelude::*;

use anime_game_core::genshin::voice_data::prelude::*;

use crate::lib::config;
use crate::ui::*;

#[derive(Clone)]
pub struct Page {
    pub page: gtk::Box,
    pub voice_packages_group: adw::PreferencesGroup,

    pub continue_button: gtk::Button,
    pub exit_button: gtk::Button,

    pub voice_packages: Vec<(VoiceLocale, adw::ActionRow, gtk::Switch)>
}

impl Page {
    pub fn new() -> anyhow::Result<Self> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run/voice_packages.ui");

        let mut result = Self {
            page: get_object(&builder, "page")?,
            voice_packages_group: get_object(&builder, "voice_packages_group")?,

            continue_button: get_object(&builder, "continue_button")?,
            exit_button: get_object(&builder, "exit_button")?,

            voice_packages: Vec::new()
        };

        let mut packages = Vec::new();

        for package in VoicePackage::list_latest().expect("Failed to list voice packages") {
            let row = adw::ActionRow::new();
            let switch = gtk::Switch::new();

            row.set_title(package.locale().to_name());
            switch.set_valign(gtk::Align::Center);

            row.add_suffix(&switch);

            result.voice_packages_group.add(&row);

            packages.push((package.locale(), row, switch));
        }

        if let Ok(config) = config::get() {
            for voice in config.game.voices {
                if let Some(voice) = VoiceLocale::from_str(voice) {
                    for (locale, _, switcher) in &packages {
                        if voice == *locale {
                            switcher.set_state(true);
                        }
                    }
                }
            }
        }

        result.voice_packages = packages;

        Ok(result)
    }

    pub fn update_config(&self, mut config: config::Config) -> config::Config {
        let mut voices = Vec::new();

        for (locale, _, switcher) in &self.voice_packages {
            if switcher.state() {
                voices.push(locale.to_code().to_string());
            }
        }

        config.game.voices = voices;

        config
    }
}
