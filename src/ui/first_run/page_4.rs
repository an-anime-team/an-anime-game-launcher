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
            sender.send(dialogue.current_folder().unwrap().to_string()).unwrap();
        }
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
        let builder = gtk::Builder::from_string(include_str!("../../../assets/ui/.dist/first_run/page_4.ui"));

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

        result.runners_folder.set_subtitle(&config.game.wine.builds);
        result.dxvk_folder.set_subtitle(&config.game.dxvk.builds);
        result.prefix_folder.set_subtitle(&config.game.wine.prefix);
        result.game_folder.set_subtitle(&config.game.path);
        result.temp_folder.set_subtitle(&match config.launcher.temp {
            Some(temp) => temp,
            None => String::from("/tmp")
        });

        result.runners_folder.connect_activated(clone!(@strong result.window as window => move |row| {
            choose_dir(row.subtitle().unwrap().to_string(), &window);
        }));

        Ok(result)
    }
}
