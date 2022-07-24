use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::glib::clone;

use std::rc::Rc;
use std::cell::Cell;
use std::io::Error;

use anime_game_core::prelude::*;

use crate::lib::config;
use crate::lib::dxvk;
use crate::lib::wine;

use crate::ui::*;
use crate::ui::traits::prelude::*;
use crate::ui::components::dxvk_row::DxvkRow;
use crate::ui::components::wine_group::WineGroup;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub window: adw::ApplicationWindow,
    pub toast_overlay: adw::ToastOverlay,

    pub page: adw::PreferencesPage,

    pub game_version: gtk::Label,
    pub patch_version: gtk::Label,

    pub wine_selected: adw::ComboRow,

    pub wine_groups: adw::PreferencesGroup,
    pub wine_recommended_only: gtk::Switch,

    pub wine_components: Rc<Vec<WineGroup>>,

    pub dxvk_selected: adw::ComboRow,

    pub dxvk_recommended_only: gtk::Switch,
    pub dxvk_vanilla: adw::ExpanderRow,
    pub dxvk_async: adw::ExpanderRow,

    pub dxvk_components: Rc<Vec<DxvkRow>>
}

impl AppWidgets {
    pub fn try_get(window: adw::ApplicationWindow, toast_overlay: adw::ToastOverlay) -> Result<Self, String> {
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/preferences_general.ui"));

        let mut result = Self {
            window,
            toast_overlay,

            page: get_object(&builder, "general_page")?,

            game_version: get_object(&builder, "game_version")?,
            patch_version: get_object(&builder, "patch_version")?,

            wine_selected: get_object(&builder, "wine_selected")?,

            wine_groups: get_object(&builder, "wine_groups")?,
            wine_recommended_only: get_object(&builder, "wine_recommended_only")?,

            wine_components: Default::default(),

            dxvk_selected: get_object(&builder, "dxvk_selected")?,

            dxvk_recommended_only: get_object(&builder, "dxvk_recommended_only")?,
            dxvk_vanilla: get_object(&builder, "dxvk_vanilla")?,
            dxvk_async: get_object(&builder, "dxvk_async")?,

            dxvk_components: Default::default()
        };

        let config = match config::get() {
            Ok(config) => config,
            Err(err) => return Err(err.to_string())
        };

        // Update wine versions lists
        let groups = match wine::List::get() {
            Ok(list) => list,
            Err(err) => return Err(err.to_string())
        };

        let mut components = Vec::new();

        for group in groups {
            let group = WineGroup::new(group);

            group.update_states(&config.game.wine.builds);

            result.wine_groups.add(&group.expander_row);

            components.push(group);
        }

        result.wine_components = Rc::new(components);

        // Update DXVK list
        let list = match dxvk::List::get() {
            Ok(list) => list,
            Err(err) => return Err(err.to_string())
        };

        let mut components = Vec::new();

        for (i, versions) in [list.vanilla, list.r#async].into_iter().enumerate() {
            for version in versions {
                let row = DxvkRow::new(version);

                row.update_state(&config.game.dxvk.builds);

                match i {
                    0 => result.dxvk_vanilla.add_row(&row.row),
                    1 => result.dxvk_async.add_row(&row.row),
                    _ => ()
                }

                components.push(row);
            }
        }

        result.dxvk_components = Rc::new(components);

        Ok(result)
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug, Clone, glib::Downgrade)]
pub enum Actions {
    DxvkPerformAction(Rc<usize>),
    WinePerformAction(Rc<(usize, usize)>),
    UpdateDxvkComboRow,
    SelectDxvkVersion(Rc<usize>),
    UpdateWineComboRow,
    SelectWineVersion(Rc<usize>),
    ToastError(Rc<(String, Error)>)
}

impl Actions {
    pub fn into_fn<T: gtk::glib::IsA<gtk::Widget>>(&self, app: &App) -> Box<dyn Fn(&T)> {
        Box::new(clone!(@strong self as action, @weak app => move |_| {
            app.update(action.clone()).expect(&format!("Failed to execute action {:?}", &action));
        }))
    }
}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default, glib::Downgrade)]
pub struct Values {
    downloaded_wine_versions: Rc<Option<Vec<wine::Version>>>,
    downloaded_dxvk_versions: Rc<Option<Vec<dxvk::Version>>>
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
    pub fn new(window: adw::ApplicationWindow, toast_overlay: adw::ToastOverlay) -> Result<Self, String> {
        let result = Self {
            widgets: AppWidgets::try_get(window, toast_overlay)?,
            values: Default::default(),
            actions: Default::default()
        }.init_events().init_actions();

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        self.widgets.wine_selected.connect_selected_notify(clone!(@weak self as this => move |combo_row| {
            if let Some(model) = combo_row.model() {
                if model.n_items() > 0 {
                    this.update(Actions::SelectWineVersion(Rc::new(combo_row.selected() as usize))).unwrap();
                }
            }
        }));

        self.widgets.dxvk_selected.connect_selected_notify(clone!(@weak self as this => move |combo_row| {
            if let Some(model) = combo_row.model() {
                if model.n_items() > 0 {
                    this.update(Actions::SelectDxvkVersion(Rc::new(combo_row.selected() as usize))).unwrap();
                }
            }
        }));

        // Set wine recommended only switcher event
        self.widgets.wine_recommended_only.connect_state_notify(clone!(@weak self as this => move |switcher| {
            for group in &*this.widgets.wine_components {
                for component in &group.version_components {
                    component.row.set_visible(if switcher.state() {
                        component.version.recommended
                    } else {
                        true
                    });
                }
            }
        }));

        // Wine install/remove buttons
        let components = &*self.widgets.wine_components;

        for (i, group) in components.into_iter().enumerate() {
            for (j, component) in (&group.version_components).into_iter().enumerate() {
                component.button.connect_clicked(Actions::WinePerformAction(Rc::new((i, j))).into_fn(&self));
            }
        }

        // Set DXVK recommended only switcher event
        self.widgets.dxvk_recommended_only.connect_state_notify(clone!(@weak self as this => move |switcher| {
            for component in &*this.widgets.dxvk_components {
                component.row.set_visible(if switcher.state() {
                    component.version.recommended
                } else {
                    true
                });
            }
        }));

        // DXVK install/remove/apply buttons
        let components = &*self.widgets.dxvk_components;

        for (i, component) in components.into_iter().enumerate() {
            component.button.connect_clicked(Actions::DxvkPerformAction(Rc::new(i)).into_fn(&self));

            component.apply_button.connect_clicked(clone!(@strong component, @weak self as this => move |_| {
                std::thread::spawn(clone!(@strong component, @strong this => move || {
                    let config = config::get().expect("Failed to load config");

                    match component.apply(&config.game.dxvk.builds, &config.game.wine.prefix) {
                        Ok(output) => println!("{}", output),
                        Err(err) => {
                            this.update(Actions::ToastError(Rc::new((
                                String::from("Failed to apply DXVK"), err
                            )))).unwrap();
                        }
                    }
                }));
            }));
        }

        self
    }

    /// Add actions processors
    /// 
    /// Changes will happen in the main thread so you can call `update` method from separate thread
    pub fn init_actions(self) -> Self {
        let (sender, receiver) = glib::MainContext::channel::<Actions>(glib::PRIORITY_DEFAULT);

        // I prefer to avoid using clone! here because it breaks my code autocompletion
        let this = self.clone();

        receiver.attach(None, move |action| {
            let mut config = config::get().expect("Failed to load config");

            // Some debug output
            println!("[general page] [update] action: {:?}", &action);

            match action {
                Actions::DxvkPerformAction(i) => {
                    let component = this.widgets.dxvk_components[*i].clone();

                    if component.is_downloaded(&config.game.dxvk.builds) {
                        if let Err(err) = component.delete(&config.game.dxvk.builds) {
                            this.update(Actions::ToastError(Rc::new((
                                String::from("Failed to delete DXVK"), err
                            )))).unwrap();
                        }

                        component.update_state(&config.game.dxvk.builds);

                        this.update(Actions::UpdateDxvkComboRow).unwrap();
                    }

                    else {
                        if let Ok(awaiter) = component.download(&config.game.dxvk.builds) {
                            awaiter.then(clone!(@strong this => move |_| {
                                match component.apply(&config.game.dxvk.builds, &config.game.wine.prefix) {
                                    Ok(output) => println!("{}", output),
                                    Err(err) => {
                                        this.update(Actions::ToastError(Rc::new((
                                            String::from("Failed to apply DXVK"), err
                                        )))).unwrap();
                                    }
                                }

                                component.update_state(&config.game.dxvk.builds);

                                this.update(Actions::UpdateDxvkComboRow).unwrap();
                            }));
                        }
                    }
                }

                Actions::WinePerformAction(version) => {
                    let component = this.widgets
                        .wine_components[version.0]
                        .version_components[version.1].clone();

                    if component.is_downloaded(&config.game.wine.builds) {
                        if let Err(err) = component.delete(&config.game.wine.builds) {
                            this.update(Actions::ToastError(Rc::new((
                                String::from("Failed to delete wine"), err
                            )))).unwrap();
                        }

                        component.update_state(&config.game.wine.builds);

                        this.update(Actions::UpdateWineComboRow).unwrap();
                    }

                    else {
                        if let Ok(awaiter) = component.download(&config.game.wine.builds) {
                            awaiter.then(clone!(@strong this => move |_| {
                                component.update_state(&config.game.wine.builds);

                                this.update(Actions::UpdateWineComboRow).unwrap();
                            }));
                        }
                    }
                }

                Actions::UpdateDxvkComboRow => {
                    let model = gtk::StringList::new(&[]);

                    let list = dxvk::List::list_downloaded(config.game.dxvk.builds)
                        .expect("Failed to list downloaded DXVK versions");

                    let mut raw_list = Vec::new();
                    let mut selected = 0;

                    for (i, group) in [list.vanilla, list.r#async].into_iter().enumerate() {
                        for version in group {
                            model.append(format!("{} {}", if i == 0 { "Vanilla" } else { "Async" }, version.version).as_str());
    
                            if let Some(curr) = &config.game.dxvk.selected {
                                if &version.name == curr {
                                    selected = raw_list.len() as u32;
                                }
                            }

                            raw_list.push(version);
                        }
                    }

                    let mut values = this.values.take();

                    values.downloaded_dxvk_versions = Rc::new(Some(raw_list));

                    this.values.set(values);

                    // This will prevent SelectDxvkVersion action to be invoked
                    let guard = this.widgets.dxvk_selected.freeze_notify();

                    // We need to return app values before we call these methods
                    // because they'll invoke SelectWineVersion action so access
                    // downloaded_wine_versions value
                    this.widgets.dxvk_selected.set_model(Some(&model));
                    this.widgets.dxvk_selected.set_selected(selected);

                    drop(guard);
                }

                Actions::SelectDxvkVersion(i) => {
                    let values = this.values.take();

                    if let Some(dxvk_versions) = &*values.downloaded_dxvk_versions {
                        let version = dxvk_versions[*i].clone();

                        if config.game.dxvk.selected != Some(version.name.clone()) {
                            config.game.dxvk.selected = Some(version.name.clone());

                            std::thread::spawn(clone!(@strong config, @strong this => move || {
                                match version.apply(&config.game.dxvk.builds, &config.game.wine.prefix) {
                                    Ok(output) => println!("{}", output),
                                    Err(err) => {
                                        this.update(Actions::ToastError(Rc::new((
                                            String::from("Failed to apply DXVK"), err
                                        )))).unwrap();
                                    }
                                }
                            }));
                        }
                    }

                    this.values.set(values);

                    config::update(config);
                }

                Actions::UpdateWineComboRow => {
                    let model = gtk::StringList::new(&["System"]);

                    let list = wine::List::list_downloaded(config.game.wine.builds)
                        .expect("Failed to list downloaded wine versions");

                    let mut selected = 0;

                    for (i, version) in (&list).into_iter().enumerate() {
                        model.append(version.title.as_str());

                        if let Some(curr) = &config.game.wine.selected {
                            if &version.name == curr {
                                selected = i as u32 + 1;
                            }
                        }
                    }

                    let mut values = this.values.take();

                    values.downloaded_wine_versions = Rc::new(Some(list));

                    this.values.set(values);

                    // This will prevent SelectWineVersion action to be invoked
                    let guard = this.widgets.wine_selected.freeze_notify();

                    // We need to return app values before we call these methods
                    // because they'll invoke SelectWineVersion action so access
                    // downloaded_wine_versions value
                    this.widgets.wine_selected.set_model(Some(&model));
                    this.widgets.wine_selected.set_selected(selected);

                    drop(guard);
                }

                Actions::SelectWineVersion(i) => {
                    let values = this.values.take();

                    if let Some(wine_versions) = &*values.downloaded_wine_versions {
                        match *i {
                            0 => config.game.wine.selected = None,
                            i => config.game.wine.selected = Some(wine_versions[i - 1].name.clone())
                        }
                    }

                    this.values.set(values);

                    config::update(config);
                }

                Actions::ToastError(toast) => {
                    let (msg, err) = (toast.0.clone(), toast.1.to_string());

                    this.toast_error(msg, err);
                }
            }

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

        // Update downloaded wine versions
        self.update(Actions::UpdateWineComboRow).unwrap();

        // Update downloaded DXVK versions
        self.update(Actions::UpdateDxvkComboRow).unwrap();

        Ok(())
    }
}

impl ToastError for App {
    fn get_toast_widgets(&self) -> (adw::ApplicationWindow, adw::ToastOverlay) {
        (self.widgets.window.clone(), self.widgets.toast_overlay.clone())
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
