use gtk4 as gtk;
use libadwaita::{self as adw, prelude::*};

use crate::lib::wine::{Version as WineVersion, List as WineList};
use crate::lib::dxvk::{Version as DxvkVersion, List as DxvkList};

use crate::ui::*;
use crate::ui::components::progress_bar::ProgressBar;

#[derive(Clone)]
pub struct Page {
    pub page: gtk::Box,

    pub wine_version: adw::ComboRow,
    pub dxvk_version: adw::ComboRow,

    pub download_button: gtk::Button,
    pub exit_button: gtk::Button,

    pub progress_bar: ProgressBar,

    pub wine_versions: Vec<WineVersion>,
    pub dxvk_versions: Vec<DxvkVersion>
}

impl Page {
    pub fn new() -> Result<Self, String> {
        let builder = gtk::Builder::from_resource("/org/app/ui/first_run/download_components.ui");

        let mut result = Self {
            page: get_object(&builder, "page")?,
            
            wine_version: get_object(&builder, "wine_version")?,
            dxvk_version: get_object(&builder, "dxvk_version")?,

            download_button: get_object(&builder, "download_button")?,
            exit_button: get_object(&builder, "exit_button")?,

            progress_bar: ProgressBar::new(
                get_object(&builder, "progress_bar")?,
                get_object(&builder, "buttons_group")?,
                get_object(&builder, "progress_bar_group")?
            ),

            wine_versions: Vec::new(),
            dxvk_versions: Vec::new()
        };

        // Add wine versions
        let model = gtk::StringList::new(&[]);

        let versions = match WineList::get() {
            Ok(versions) => versions,
            Err(err) => return Err(err.to_string())
        };

        for version in &versions[0].versions {
            if version.recommended {
                model.append(&version.title);

                result.wine_versions.push(version.clone());
            }
        }

        result.wine_version.set_model(Some(&model));

        // Add DXVK versions
        let model = gtk::StringList::new(&[]);

        let versions = match DxvkList::get() {
            Ok(versions) => versions,
            Err(err) => return Err(err.to_string())
        };

        for version in &versions.vanilla {
            if version.recommended {
                model.append(&version.version);

                result.dxvk_versions.push(version.clone());
            }
        }

        result.dxvk_version.set_model(Some(&model));

        Ok(result)
    }

    pub fn get_wine_version(&self) -> &WineVersion {
        &self.wine_versions[self.wine_version.selected() as usize]
    }

    pub fn get_dxvk_version(&self) -> &DxvkVersion {
        &self.dxvk_versions[self.dxvk_version.selected() as usize]
    }
}
