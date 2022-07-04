use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk4::glib;
use gtk4::glib::clone;

use std::rc::Rc;
use std::cell::Cell;
use std::io::Error;

use anime_game_core::prelude::*;

use crate::ui::get_object;
use crate::lib::config;
use crate::lib::dxvk;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub page: adw::PreferencesPage,

    pub game_version: gtk::Label,
    pub patch_version: gtk::Label,

    pub dxvk_recommended_only: gtk::Switch,
    pub dxvk_vanilla: adw::ExpanderRow,
    pub dxvk_async: adw::ExpanderRow,

    pub dxvk_components: Rc<Vec<(adw::ActionRow, dxvk::Version)>>
}

impl AppWidgets {
    fn try_get() -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences_general.ui"));

        let mut result = Self {
            page: get_object(&builder, "general_page")?,

            game_version: get_object(&builder, "game_version")?,
            patch_version: get_object(&builder, "patch_version")?,

            dxvk_recommended_only: get_object(&builder, "dxvk_recommended_only")?,
            dxvk_vanilla: get_object(&builder, "dxvk_vanilla")?,
            dxvk_async: get_object(&builder, "dxvk_async")?,

            dxvk_components: Default::default()
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

        result.dxvk_components = Rc::new(components);

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug)]
pub enum Actions {

}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default, glib::Downgrade)]
pub struct Values;

/// The main application structure
/// 
/// `Default` macro automatically calls `AppWidgets::default`, i.e. loads UI file and reference its widgets
/// 
/// `Rc<Cell<Values>>` means this:
/// - `Rc` addeds ability to reference the same value from various clones of the structure.
///   This will guarantee us that inner `Cell<Values>` is the same for all the `App::clone()` values
/// - `Cell` addeds inner mutability to its value, so we can mutate it even without mutable reference.
/// 
/// So we have a shared reference to some value that can be changed without mutable reference.
/// That's what we need and what we use in `App::update` method
#[derive(Clone, glib::Downgrade)]
pub struct App {
    widgets: AppWidgets,
    values: Rc<Cell<Values>>
}

impl App {
    /// Create new application
    pub fn new() -> Result<Self, String> {
        let result = Self {
            widgets: AppWidgets::try_get()?,
            values: Default::default()
        }.init_events();

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Set DXVK recommended only switcher event
        self.widgets.dxvk_recommended_only.connect_state_notify(clone!(@weak self as this => move |switcher| {
            for (component, version) in &*this.widgets.dxvk_components {
                component.set_visible(if switcher.state() {
                    version.recommended
                } else {
                    true
                });
            }
        }));

        self
    }

    /// Update widgets state by calling some action
    pub fn update(&self, action: Actions) {
        /*let values = self.values.take();

        match action {
            
        }

        self.values.set(values);*/
    }

    pub fn title() -> String {
        String::from("General")
    }

    pub fn get_page(&self) -> adw::PreferencesPage {
        self.widgets.page.clone()
    }

    /// This method is being called by the `PreferencesStack::update`
    pub fn prepare(&self, status_page: &adw::StatusPage) -> Result<(), Error> {
        let config = config::get()?;
        let game = Game::new(config.game.path);

        self.widgets.game_version.set_tooltip_text(None);
        self.widgets.patch_version.set_tooltip_text(None);

        // Update game version
        status_page.set_description(Some("Updating game info..."));

        match game.try_get_diff()? {
            VersionDiff::Latest(version) => {
                self.widgets.game_version.set_label(&version.to_string());
            },
            VersionDiff::Diff { current, latest, .. } => {
                self.widgets.game_version.set_label(&current.to_string());
                self.widgets.game_version.set_css_classes(&["warning"]);

                self.widgets.game_version.set_tooltip_text(Some(&format!("Game update available: {} -> {}", current, latest)));
            },
            VersionDiff::Outdated { current, latest } => {
                self.widgets.game_version.set_label(&current.to_string());
                self.widgets.game_version.set_css_classes(&["error"]);

                self.widgets.game_version.set_tooltip_text(Some(&format!("Game is too outdated and can't be updated. Latest version: {}", latest)));
            },
            VersionDiff::NotInstalled { .. } => {
                self.widgets.game_version.set_label("not installed");
                self.widgets.game_version.set_css_classes(&[]);
            }
        }

        // Update patch version
        status_page.set_description(Some("Updating patch info..."));

        match Patch::try_fetch(config.patch.servers)? {
            Patch::NotAvailable => {
                self.widgets.patch_version.set_label("not available");
                self.widgets.patch_version.set_css_classes(&["error"]);

                self.widgets.patch_version.set_tooltip_text(Some("Patch is not available"));
            },
            Patch::Outdated { current, latest, .. } => {
                self.widgets.patch_version.set_label("outdated");
                self.widgets.patch_version.set_css_classes(&["warning"]);

                self.widgets.patch_version.set_tooltip_text(Some(&format!("Patch is outdated ({} -> {})", current, latest)));
            },
            Patch::Preparation { .. } => {
                self.widgets.patch_version.set_label("preparation");
                self.widgets.patch_version.set_css_classes(&["warning"]);

                self.widgets.patch_version.set_tooltip_text(Some("Patch is in preparation state and will be available later"));
            },
            Patch::Testing { version, .. } => {
                self.widgets.patch_version.set_label(&version.to_string());
                self.widgets.patch_version.set_css_classes(&["warning"]);

                self.widgets.patch_version.set_tooltip_text(Some("Patch is in testing phase"));
            },
            Patch::Available { version, .. } => {
                self.widgets.patch_version.set_label(&version.to_string());
                self.widgets.patch_version.set_css_classes(&["success"]);
            }
        }

        Ok(())
    }
}
