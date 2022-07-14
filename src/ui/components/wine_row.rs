use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::Align;

use crate::lib::wine::Version;

#[derive(Debug, Clone)]
pub struct WineRow {
    pub version: Version,

    pub row: adw::ActionRow,
    pub button: gtk::Button,
    pub progress_bar: gtk::ProgressBar
}

impl WineRow {
    pub fn new(version: Version) -> Self {
        let row = adw::ActionRow::new();
        let button = gtk::Button::new();

        row.set_title(&version.title);
        row.set_visible(version.recommended);

        button.set_icon_name("document-save-symbolic");
        button.set_valign(gtk::Align::Center);
        button.add_css_class("flat");

        row.add_suffix(&button);

        let progress_bar = gtk::ProgressBar::new();

        progress_bar.set_text(Some("Downloading: 0%"));
        progress_bar.set_show_text(true);

        progress_bar.set_width_request(200);
        progress_bar.set_valign(Align::Center);
        progress_bar.set_visible(false);

        row.add_suffix(&progress_bar);

        Self {
            version,
            row,
            button,
            progress_bar
        }
    }

    pub fn download(&self) {
        let (sender, receiver) = glib::MainContext::channel::<i32>(glib::PRIORITY_DEFAULT);
        let this = self.clone();

        this.progress_bar.set_visible(true);
        this.button.set_visible(false);

        receiver.attach(None, move |fraction| {
            this.progress_bar.set_fraction(fraction as f64 / 100f64);
            this.progress_bar.set_text(Some(&format!("Downloading: {}%", fraction)));

            if fraction == 100 {
                this.progress_bar.set_visible(false);
                this.button.set_visible(true);
            }

            glib::Continue(true)
        });

        std::thread::spawn(move || {
            for i in 1..101 {
                std::thread::sleep(std::time::Duration::from_millis(150));

                sender.send(i);
            }
        });
    }
}
