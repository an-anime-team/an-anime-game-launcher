use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::Error;

use anime_game_core::prelude::*;

use crate::ui::get_object;
use crate::lib::config;

mod general_page;
mod enhanced_page;

pub mod pages {
    pub use super::general_page::Page as GeneralPage;
    pub use super::enhanced_page::Page as EnhancedPage;
}

#[derive(Clone)]
pub struct PreferencesStack {
    pub preferences: gtk::Box,
    pub preferences_go_back: gtk::Button,
    pub stack: gtk::Stack,
    pub general_page: pages::GeneralPage,
    pub enhanced_page: pages::EnhancedPage
}

impl PreferencesStack {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences.ui"));

        let result = Self {
            preferences: get_object(&builder, "preferences")?,
            preferences_go_back: get_object(&builder, "preferences_go_back")?,
            stack: get_object(&builder, "stack")?,
            general_page: pages::GeneralPage::new()?,
            enhanced_page: pages::EnhancedPage::new()?
        };

        result.stack.add_titled(&result.general_page.page, None, &pages::GeneralPage::title());
        result.stack.add_titled(&result.enhanced_page.page, None, &pages::EnhancedPage::title());

        Ok(result)
    }

    /// Update page info before opening it
    /// 
    /// Being called from the `MainApp` struct
    /// 
    /// TODO: do it asynchronously
    pub fn update(&self) -> Result<(), Error> {
        let config = config::get()?;
        let game = Game::new(config.game.path);

        self.general_page.game_version.set_tooltip_text(None);
        self.general_page.patch_version.set_tooltip_text(None);

        match game.try_get_diff()? {
            VersionDiff::Latest(version) => {
                self.general_page.game_version.set_label(&version.to_string());
            },
            VersionDiff::Diff { current, latest, .. } => {
                self.general_page.game_version.set_label(&current.to_string());
                self.general_page.game_version.set_css_classes(&["warning"]);

                self.general_page.game_version.set_tooltip_text(Some(&format!("Game update available: {} -> {}", current, latest)));
            },
            VersionDiff::Outdated { current, latest } => {
                self.general_page.game_version.set_label(&current.to_string());
                self.general_page.game_version.set_css_classes(&["error"]);

                self.general_page.game_version.set_tooltip_text(Some(&format!("Game is too outdated and can't be updated. Latest version: {}", latest)));
            },
            VersionDiff::NotInstalled { .. } => {
                self.general_page.game_version.set_label("not installed");
                self.general_page.game_version.set_css_classes(&[]);
            }
        }

        match Patch::try_fetch(config.patch.servers)? {
            Patch::NotAvailable => {
                self.general_page.patch_version.set_label("not available");
                self.general_page.patch_version.set_css_classes(&["error"]);

                self.general_page.patch_version.set_tooltip_text(Some("Patch is not available"));
            },
            Patch::Outdated { current, latest, .. } => {
                self.general_page.patch_version.set_label("outdated");
                self.general_page.patch_version.set_css_classes(&["warning"]);

                self.general_page.patch_version.set_tooltip_text(Some(&format!("Patch is outdated ({} -> {})", current, latest)));
            },
            Patch::Preparation { .. } => {
                self.general_page.patch_version.set_label("preparation");
                self.general_page.patch_version.set_css_classes(&["warning"]);

                self.general_page.patch_version.set_tooltip_text(Some("Patch is in preparation state and will be available later"));
            },
            Patch::Testing { version, .. } => {
                self.general_page.patch_version.set_label(&version.to_string());
                self.general_page.patch_version.set_css_classes(&["warning"]);

                self.general_page.patch_version.set_tooltip_text(Some("Patch is in testing phase"));
            },
            Patch::Available { version, .. } => {
                self.general_page.patch_version.set_label(&version.to_string());
                self.general_page.patch_version.set_css_classes(&["success"]);
            }
        }

        Ok(())
    }
}
