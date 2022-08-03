use gtk4 as gtk;
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::glib::clone;

use wait_not_await::Await;

use crate::lib::config;
use crate::ui::*;

pub fn choose_dir<T: IsA<gtk::Window>>(current_folder: String, parent: &T) -> Await<String> {
    let dialogue = gtk::FileChooserDialog::new(
        Some("Select folder"),
        Some(parent),
        gtk::FileChooserAction::SelectFolder,
        &[("Select", gtk::ResponseType::Accept)]
    );

    dialogue.set_current_folder(Some(&gtk::gio::File::for_path(current_folder))).unwrap();

    let (sender, receiver) = std::sync::mpsc::channel();

    dialogue.connect_response(move |dialogue, response| {
        if response == gtk::ResponseType::Accept {
            sender.send(dialogue.current_folder().unwrap().path().unwrap().to_str().unwrap().to_string()).unwrap();
        }

        dialogue.close();
    });

    dialogue.show();

    Await::new(move || {
        receiver.recv().unwrap()
    })
}

#[derive(Clone)]
pub struct Page {
    pub window: gtk::Window,
    pub page: gtk::Box,

    pub runners_folder: adw::ActionRow,
    pub dxvk_folder: adw::ActionRow,
    pub prefix_folder: adw::ActionRow,
    pub game_folder: adw::ActionRow,
    pub temp_folder: adw::ActionRow,

    pub continue_button: gtk::Button,
    pub exit_button: gtk::Button
}

impl Page {
    pub fn new(window: gtk::Window) -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run/default_paths.ui");

        let result = Self {
            window,
            page: get_object(&builder, "page")?,

            runners_folder: get_object(&builder, "runners_folder")?,
            dxvk_folder: get_object(&builder, "dxvk_folder")?,
            prefix_folder: get_object(&builder, "prefix_folder")?,
            game_folder: get_object(&builder, "game_folder")?,
            temp_folder: get_object(&builder, "temp_folder")?,

            continue_button: get_object(&builder, "continue_button")?,
            exit_button: get_object(&builder, "exit_button")?
        };

        let config = match config::get() {
            Ok(config) => config,
            Err(err) => return Err(err.to_string())
        };

        // Add paths to subtitles
        result.runners_folder.set_subtitle(&config.game.wine.builds);
        result.dxvk_folder.set_subtitle(&config.game.dxvk.builds);
        result.prefix_folder.set_subtitle(&config.game.wine.prefix);
        result.game_folder.set_subtitle(&config.game.path);
        result.temp_folder.set_subtitle(&match config.launcher.temp {
            Some(temp) => temp,
            None => String::from("/tmp")
        });

        // Connect path selection events
        result.connect_activated(&result.runners_folder);
        result.connect_activated(&result.dxvk_folder);
        result.connect_activated(&result.prefix_folder);
        result.connect_activated(&result.game_folder);
        result.connect_activated(&result.temp_folder);

        Ok(result)
    }

    fn connect_activated(&self, row: &adw::ActionRow) {
        row.connect_activated(clone!(@strong self.window as window => move |row| {
            let (sender, receiver) = glib::MainContext::channel::<String>(glib::PRIORITY_DEFAULT);

            choose_dir(row.subtitle().unwrap().to_string(), &window).then(move |path| {
                sender.send(path.clone()).unwrap();
            });

            let row = row.clone();

            receiver.attach(None, move |path| {
                row.set_subtitle(&path);

                glib::Continue(false)
            });
        }));
    }

    pub fn update_config(&self, mut config: config::Config) -> config::Config {
        config.game.wine.builds = self.runners_folder.subtitle().unwrap().to_string();
        config.game.dxvk.builds = self.dxvk_folder.subtitle().unwrap().to_string();
        config.game.wine.prefix = self.prefix_folder.subtitle().unwrap().to_string();
        config.game.path        = self.game_folder.subtitle().unwrap().to_string();
        config.launcher.temp    = Some(self.temp_folder.subtitle().unwrap().to_string());

        config
    }
}
