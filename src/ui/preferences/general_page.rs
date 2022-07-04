use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::Error;

use anime_game_core::prelude::*;

use crate::ui::get_object;
use crate::lib::config;
use crate::lib::dxvk;

#[derive(Clone)]
pub struct Page {
    pub page: adw::PreferencesPage,

    pub game_version: gtk::Label,
    pub patch_version: gtk::Label,

    pub dxvk_recommended_only: gtk::Switch,
    pub dxvk_vanilla: adw::ExpanderRow,
    pub dxvk_async: adw::ExpanderRow
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences_general.ui"));

        let result = Self {
            page: get_object(&builder, "general_page")?,

            game_version: get_object(&builder, "game_version")?,
            patch_version: get_object(&builder, "patch_version")?,

            dxvk_recommended_only: get_object(&builder, "dxvk_recommended_only")?,
            dxvk_vanilla: get_object(&builder, "dxvk_vanilla")?,
            dxvk_async: get_object(&builder, "dxvk_async")?
        };

        // Update DXVK list
        let list = match dxvk::List::get() {
            Ok(list) => list,
            Err(err) => return Err(err.to_string())
        };

        let mut components = Vec::new();

        for version in list.vanilla {
            let row = adw::ActionRow::new();
            let button = gtk::Button::new();

            row.set_title(&version.version);
            row.set_visible(version.recommended);

            button.set_icon_name("document-save-symbolic");
            button.set_valign(gtk::Align::Center);
            button.add_css_class("flat");

            row.add_suffix(&button);

            result.dxvk_vanilla.add_row(&row);
            components.push((row, version));
        }

        for version in list.r#async {
            let row = adw::ActionRow::new();
            let button = gtk::Button::new();

            row.set_title(&version.version);
            row.set_visible(version.recommended);

            button.set_icon_name("document-save-symbolic");
            button.set_valign(gtk::Align::Center);
            button.add_css_class("flat");

            row.add_suffix(&button);

            result.dxvk_async.add_row(&row);
            components.push((row, version));
        }
        
        // Set DXVK recommended only switcher event
        result.dxvk_recommended_only.connect_state_notify(move |switcher| {
            for (component, version) in &components {
                component.set_visible(if switcher.state() {
                    version.recommended
                } else {
                    true
                });
            }
        });

        Ok(result)
    }

    pub fn title() -> String {
        String::from("General")
    }

    /// This method is being called by the `PreferencesStack::update`
    pub fn update(&self, status_page: &adw::StatusPage) -> Result<(), Error> {
        let config = config::get()?;
        let game = Game::new(config.game.path);

        self.game_version.set_tooltip_text(None);
        self.patch_version.set_tooltip_text(None);

        // Update game version
        status_page.set_description(Some("Updating game info..."));

        match game.try_get_diff()? {
            VersionDiff::Latest(version) => {
                self.game_version.set_label(&version.to_string());
            },
            VersionDiff::Diff { current, latest, .. } => {
                self.game_version.set_label(&current.to_string());
                self.game_version.set_css_classes(&["warning"]);

                self.game_version.set_tooltip_text(Some(&format!("Game update available: {} -> {}", current, latest)));
            },
            VersionDiff::Outdated { current, latest } => {
                self.game_version.set_label(&current.to_string());
                self.game_version.set_css_classes(&["error"]);

                self.game_version.set_tooltip_text(Some(&format!("Game is too outdated and can't be updated. Latest version: {}", latest)));
            },
            VersionDiff::NotInstalled { .. } => {
                self.game_version.set_label("not installed");
                self.game_version.set_css_classes(&[]);
            }
        }

        // Update patch version
        status_page.set_description(Some("Updating patch info..."));

        match Patch::try_fetch(config.patch.servers)? {
            Patch::NotAvailable => {
                self.patch_version.set_label("not available");
                self.patch_version.set_css_classes(&["error"]);

                self.patch_version.set_tooltip_text(Some("Patch is not available"));
            },
            Patch::Outdated { current, latest, .. } => {
                self.patch_version.set_label("outdated");
                self.patch_version.set_css_classes(&["warning"]);

                self.patch_version.set_tooltip_text(Some(&format!("Patch is outdated ({} -> {})", current, latest)));
            },
            Patch::Preparation { .. } => {
                self.patch_version.set_label("preparation");
                self.patch_version.set_css_classes(&["warning"]);

                self.patch_version.set_tooltip_text(Some("Patch is in preparation state and will be available later"));
            },
            Patch::Testing { version, .. } => {
                self.patch_version.set_label(&version.to_string());
                self.patch_version.set_css_classes(&["warning"]);

                self.patch_version.set_tooltip_text(Some("Patch is in testing phase"));
            },
            Patch::Available { version, .. } => {
                self.patch_version.set_label(&version.to_string());
                self.patch_version.set_css_classes(&["success"]);
            }
        }

        Ok(())
    }
}
