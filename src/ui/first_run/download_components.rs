use adw::prelude::*;

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
    pub dxvk_versions: Vec<DxvkVersion>,

    system_wine_available: bool
}

impl Page {
    pub fn new() -> anyhow::Result<Self> {
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
            dxvk_versions: Vec::new(),

            system_wine_available: crate::lib::is_available("wine64")
        };

        // Add wine versions
        let model = gtk::StringList::new(&[]);

        if result.system_wine_available {
            model.append("System");
        }

        for version in &WineList::get()[0].versions {
            if version.recommended {
                model.append(&version.title);

                result.wine_versions.push(version.clone());
            }
        }

        result.wine_version.set_model(Some(&model));

        // We're not recommending user to use system wine
        // and suggest to download some wine build better for gaming
        if result.system_wine_available {
            result.wine_version.set_selected(1);
        }

        // Add DXVK versions
        let model = gtk::StringList::new(&[]);

        for version in &DxvkList::get()[0].versions {
            if version.recommended {
                model.append(&version.version);

                result.dxvk_versions.push(version.clone());
            }
        }

        result.dxvk_version.set_model(Some(&model));

        Ok(result)
    }

    /// Get selected wine version
    /// 
    /// `None` means `System`
    pub fn get_wine_version(&self) -> Option<WineVersion> {
        if self.system_wine_available {
            match self.wine_version.selected() {
                0 => None,
                i => Some(self.wine_versions[i as usize - 1].clone())
            }
        }
        
        else {
            Some(self.wine_versions[self.wine_version.selected() as usize].clone())
        }
    }

    pub fn get_dxvk_version(&self) -> &DxvkVersion {
        &self.dxvk_versions[self.dxvk_version.selected() as usize]
    }
}
