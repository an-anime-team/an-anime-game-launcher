use gtk4 as gtk;

use std::process::{Command, Stdio};

use crate::ui::*;

#[derive(Clone)]
pub struct Page {
    pub page: gtk::Box,

    pub pkg_pacman: gtk::Box,
    pub pkg_apt: gtk::Box,
    pub pkg_dnf: gtk::Box,

    pub check_button: gtk::Button,
    pub exit_button: gtk::Button
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run/dependencies.ui");

        let result = Self {
            page: get_object(&builder, "page")?,

            pkg_pacman: get_object(&builder, "pkg_pacman")?,
            pkg_apt: get_object(&builder, "pkg_apt")?,
            pkg_dnf: get_object(&builder, "pkg_dnf")?,

            check_button: get_object(&builder, "check_button")?,
            exit_button: get_object(&builder, "exit_button")?
        };

        // Decide which packaging format used in system
        match Command::new("pacman").stdout(Stdio::null()).spawn() {
            Ok(_) => result.pkg_pacman.show(),

            Err(_) => match Command::new("apt").stdout(Stdio::null()).spawn() {
                Ok(_) => result.pkg_apt.show(),

                Err(_) => match Command::new("dnf").stdout(Stdio::null()).spawn() {
                    Ok(_) => result.pkg_dnf.show(),

                    Err(_) => {
                        result.pkg_pacman.show();
                        result.pkg_apt.show();
                        result.pkg_dnf.show();
                    }
                }
            }
        }

        Ok(result)
    }
}
