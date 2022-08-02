use gtk4 as gtk;
use libadwaita::{self as adw, prelude::*};

use gtk4::glib;
use gtk4::glib::clone;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::Cell;
use std::io::Error;

use crate::ui::get_object;
use crate::lib::config;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub page: adw::PreferencesPage,

    pub command: gtk::Entry,

    pub variables: adw::PreferencesGroup,

    pub name: gtk::Entry,
    pub value: gtk::Entry,
    pub add: gtk::Button
}

impl AppWidgets {
    fn try_get() -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/preferences/environment.ui");

        let result = Self {
            page: get_object(&builder, "page")?,

            command: get_object(&builder, "command")?,

            variables: get_object(&builder, "variables")?,

            name: get_object(&builder, "name")?,
            value: get_object(&builder, "value")?,
            add: get_object(&builder, "add")?
        };

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug, Clone)]
pub enum Actions {
    Add(Rc<(String, String)>),
    Delete(Rc<String>)
}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default)]
pub struct Values {
    pub rows: HashMap<String, adw::ActionRow>
}

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
        let this = self.clone();

        self.widgets.add.connect_clicked(move |_| {
            let name = this.widgets.name.text().to_string();
            let value = this.widgets.value.text().to_string();

            this.update(Actions::Add(Rc::new((name, value)))).unwrap();
        });

        self.widgets.command.connect_changed(move |entry| {
            if let Ok(mut config) = config::get() {
                let command = entry.text().to_string();

                config.game.command = if command.is_empty() {
                    None
                } else {
                    Some(command)
                };

                config::update(config);
            }
        });

        self
    }

    /// Add actions processors
    /// 
    /// Changes will happen in the main thread so you can call `update` method from separate thread
    fn init_actions(self) -> Self {
        let (sender, receiver) = glib::MainContext::channel::<Actions>(glib::PRIORITY_DEFAULT);

        // I prefer to avoid using clone! here because it breaks my code autocompletion
        let this = self.clone();

        receiver.attach(None, move |action| {
            let mut config = config::get().expect("Failed to load config");
            let mut values = this.values.take();

            // Some debug output
            println!("[environment page] [update] action: {:?}", &action);

            match action {
                Actions::Add(strs) => {
                    let (name, value) = &*strs;

                    if !name.is_empty() && !value.is_empty() {
                        if !values.rows.contains_key(name) {
                            config.game.environment.insert(name.clone(), value.clone());

                            let row = adw::ActionRow::new();

                            row.set_title(name);
                            row.set_subtitle(value);

                            let button = gtk::Button::new();

                            button.set_icon_name("user-trash-symbolic");
                            button.set_valign(gtk::Align::Center);
                            button.add_css_class("flat");

                            button.connect_clicked(clone!(@weak this, @strong name => move |_| {
                                this.update(Actions::Delete(Rc::new(name.clone()))).unwrap();
                            }));

                            row.add_suffix(&button);

                            this.widgets.variables.add(&row);

                            values.rows.insert(name.clone(), row);

                            this.widgets.name.set_text("");
                            this.widgets.value.set_text("");
                        }
                    }
                }

                Actions::Delete(name) => {
                    let name = &*name;

                    if let Some(widget) = values.rows.get(name) {
                        this.widgets.variables.remove(widget);
                    }

                    values.rows.remove(name);
                    config.game.environment.remove(name);
                }
            }

            config::update(config);

            this.values.set(values);

            glib::Continue(true)
        });

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
        String::from("Environment")
    }

    pub fn get_page(&self) -> adw::PreferencesPage {
        self.widgets.page.clone()
    }

    /// This method is being called by the `PreferencesStack::update`
    pub fn prepare(&self, status_page: &adw::StatusPage) -> Result<(), Error> {
        let config = config::get()?;

        status_page.set_description(Some("Loading environment..."));

        for (name, value) in config.game.environment {
            self.update(Actions::Add(Rc::new((name, value)))).unwrap();
        }

        Ok(())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
