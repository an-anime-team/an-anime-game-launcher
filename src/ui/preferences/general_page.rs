use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::glib::clone;
use gtk::Align;

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

    pub dxvk_components: Rc<Vec<(adw::ActionRow, gtk::Button, dxvk::Version)>>
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

        for (i, versions) in [list.vanilla, list.r#async].into_iter().enumerate() {
            for version in versions {
                let row = adw::ActionRow::new();
                let button = gtk::Button::new();

                row.set_title(&version.version);
                row.set_visible(version.recommended);

                button.set_icon_name("document-save-symbolic");
                button.set_valign(gtk::Align::Center);
                button.add_css_class("flat");

                row.add_suffix(&button);

                match i {
                    0 => result.dxvk_vanilla.add_row(&row),
                    1 => result.dxvk_async.add_row(&row),
                    _ => ()
                }

                components.push((row, button, version));
            }
        }

        result.dxvk_components = Rc::new(components);

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug, glib::Downgrade)]
pub enum Actions {
    DownloadDXVK(Rc<usize>)
}

impl Actions {
    pub fn into_fn<T: gtk::glib::IsA<gtk::Widget>>(&self, app: &App) -> Box<dyn Fn(&T)> {
        Box::new(clone!(@weak self as action, @strong app => move |_| {
            app.update(action);
        }))
    }
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
    values: Rc<Cell<Values>>,
    actions: Rc<Cell<Option<glib::Sender<Actions>>>>
}

impl App {
    /// Create new application
    pub fn new() -> Result<Self, String> {
        let result = Self {
            widgets: AppWidgets::try_get()?,
            values: Default::default(),
            actions: Default::default()
        }.init_events().init_actions();

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Set DXVK recommended only switcher event
        self.widgets.dxvk_recommended_only.connect_state_notify(clone!(@strong self as this => move |switcher| {
            for (component, _, version) in &*this.widgets.dxvk_components {
                component.set_visible(if switcher.state() {
                    version.recommended
                } else {
                    true
                });
            }
        }));

        // DXVK install/remove buttons
        let components = (*self.widgets.dxvk_components).clone();

        for (i, (_, button, _)) in components.into_iter().enumerate() {
            button.connect_clicked(Actions::DownloadDXVK(Rc::new(i)).into_fn(&self));
        }

        self
    }

    /// Add actions processors
    /// 
    /// Changes will happen in the main thread so you can call `update` method from separate thread
    pub fn init_actions(self) -> Self {
        let (sender, receiver) = glib::MainContext::channel::<Actions>(glib::PRIORITY_DEFAULT);

        receiver.attach(None, clone!(@strong self as this => move |action| {
            let values = this.values.take();

            // Some debug output
            println!("[update] action: {:?}, values: {:?}", &action, &values);

            match action {
                Actions::DownloadDXVK(i) => {
                    println!("123");

                    let (row, button, version) = &this.widgets.dxvk_components[*i];
    
                    let progress_bar = gtk::ProgressBar::new();
    
                    progress_bar.set_text(Some("Downloading: 0%"));
                    progress_bar.set_show_text(true);
                    
                    progress_bar.set_width_request(200);
                    progress_bar.set_valign(Align::Center);
    
                    button.set_visible(false);
    
                    row.add_suffix(&progress_bar);
    
                    row.remove(&progress_bar);
                    button.set_visible(true);
                }
            }

            this.values.set(values);

            glib::Continue(true)
        }));

        self.actions.set(Some(sender));

        self
    }

    /// Update widgets state by calling some action
    pub fn update(&self, action: Actions) -> Result<(), std::sync::mpsc::SendError<Actions>> {
        let actions = self.actions.take();
        
        let result = match &actions {
            Some(sender) => Ok(sender.send(action)?),
            None => Ok(())
        };

        self.actions.set(actions);

        result
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

unsafe impl Send for App {}
