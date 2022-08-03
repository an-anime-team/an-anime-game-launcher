use gtk4 as gtk;
use libadwaita::{self as adw, prelude::*};

use gtk::glib;

use crate::ui::get_object;
use crate::lib::config;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::try_get` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone, glib::Downgrade)]
pub struct AppWidgets {
    pub window: adw::PreferencesWindow,

    pub game_width: gtk::Entry,
    pub game_height: gtk::Entry,

    pub gamescope_width: gtk::Entry,
    pub gamescope_height: gtk::Entry,

    pub framerate_limit: gtk::Entry,
    pub framerate_unfocused_limit: gtk::Entry,
    pub integer_scaling: gtk::Switch,
    pub nvidia_image_scaling: gtk::Switch,

    pub borderless: gtk::ToggleButton,
    pub fullscreen: gtk::ToggleButton
}

impl AppWidgets {
    fn try_get(window: &adw::ApplicationWindow) -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/preferences/gamescope.ui");

        let result = Self {
            window: get_object(&builder, "window")?,

            game_width: get_object(&builder, "game_width")?,
            game_height: get_object(&builder, "game_height")?,

            gamescope_width: get_object(&builder, "gamescope_width")?,
            gamescope_height: get_object(&builder, "gamescope_height")?,

            framerate_limit: get_object(&builder, "framerate_limit")?,
            framerate_unfocused_limit: get_object(&builder, "framerate_unfocused_limit")?,
            integer_scaling: get_object(&builder, "integer_scaling")?,
            nvidia_image_scaling: get_object(&builder, "nvidia_image_scaling")?,

            borderless: get_object(&builder, "borderless")?,
            fullscreen: get_object(&builder, "fullscreen")?
        };

        result.window.set_transient_for(Some(window));

        Ok(result)
    }
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
    widgets: AppWidgets
}

impl App {
    /// Create new application
    pub fn new(window: &adw::ApplicationWindow) -> Result<Self, String> {
        let result = Self {
            widgets: AppWidgets::try_get(window)?
        }.init_events();

        Ok(result)
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Game width
        self.widgets.game_width.connect_changed(move |entry| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.game.width = entry.text().parse().unwrap_or(0);

                config::update(config);
            }
        });

        // Game height
        self.widgets.game_height.connect_changed(move |entry| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.game.height = entry.text().parse().unwrap_or(0);

                config::update(config);
            }
        });

        // Gamescope width
        self.widgets.gamescope_width.connect_changed(move |entry| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.gamescope.width = entry.text().parse().unwrap_or(0);

                config::update(config);
            }
        });

        // Gamescope height
        self.widgets.gamescope_height.connect_changed(move |entry| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.gamescope.height = entry.text().parse().unwrap_or(0);

                config::update(config);
            }
        });

        // Framerate focused
        self.widgets.framerate_limit.connect_changed(move |entry| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.framerate.focused = entry.text().parse().unwrap_or(0);

                config::update(config);
            }
        });

        // Framerate unfocused
        self.widgets.framerate_unfocused_limit.connect_changed(move |entry| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.framerate.unfocused = entry.text().parse().unwrap_or(0);

                config::update(config);
            }
        });

        // Use integer scaling
        self.widgets.integer_scaling.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.integer_scaling = switch.state();

                config::update(config);
            }
        });

        // Use NIS (Nvidia Image Scaling)
        self.widgets.nvidia_image_scaling.connect_state_notify(move |switch| {
            if let Ok(mut config) = config::get() {
                config.game.enhancements.gamescope.nvidia_image_scaling = switch.state();

                config::update(config);
            }
        });

        // Window type

        let borderless = self.widgets.borderless.clone();
        let fullscreen = self.widgets.fullscreen.clone();

        // Window type (Borderless)
        self.widgets.borderless.connect_clicked(move |button| {
            if !button.is_active() {
                button.activate();
            }

            else {
                fullscreen.set_active(false);

                if let Ok(mut config) = config::get() {
                    config.game.enhancements.gamescope.window_type = if button.is_active() {
                        config::WindowType::Borderless
                    } else {
                        config::WindowType::Fullscreen
                    };
    
                    config::update(config);
                }
            }
        });

        // Window type (Fullscreen)
        self.widgets.fullscreen.connect_clicked(move |button| {
            if !button.is_active() {
                button.activate();
            }

            else {
                borderless.set_active(false);

                if let Ok(mut config) = config::get() {
                    config.game.enhancements.gamescope.window_type = if button.is_active() {
                        config::WindowType::Fullscreen
                    } else {
                        config::WindowType::Borderless
                    };
    
                    config::update(config);
                }
            }
        });

        self
    }

    /// This method is being called by the `EnhancementsPage::prepare`
    pub fn prepare(&self, status_page: &adw::StatusPage) -> std::io::Result<()> {
        let config = config::get()?;

        status_page.set_description(Some("Loading gamescope..."));

        fn set_text(widget: &gtk::Entry, value: u16) {
            widget.set_text(&if value == 0 { String::new() } else { value.to_string() });
        }

        set_text(&self.widgets.game_width, config.game.enhancements.gamescope.game.width);
        set_text(&self.widgets.game_height, config.game.enhancements.gamescope.game.height);

        set_text(&self.widgets.gamescope_width, config.game.enhancements.gamescope.gamescope.width);
        set_text(&self.widgets.gamescope_height, config.game.enhancements.gamescope.gamescope.height);

        set_text(&self.widgets.framerate_limit, config.game.enhancements.gamescope.framerate.focused);
        set_text(&self.widgets.framerate_unfocused_limit, config.game.enhancements.gamescope.framerate.unfocused);

        self.widgets.integer_scaling.set_state(config.game.enhancements.gamescope.integer_scaling);
        self.widgets.nvidia_image_scaling.set_state(config.game.enhancements.gamescope.nvidia_image_scaling);

        match config.game.enhancements.gamescope.window_type {
            config::WindowType::Borderless => self.widgets.borderless.set_active(true),
            config::WindowType::Fullscreen => self.widgets.fullscreen.set_active(true)
        };

        Ok(())
    }

    pub fn show(&self) {
        self.widgets.window.show();
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
