use libadwaita::{self as adw, prelude::*};

use crate::lib::dxvk::Group;
use super::dxvk_row::DxvkRow;

#[derive(Debug, Clone)]
pub struct DxvkGroup {
    pub group: Group,
    pub version_components: Vec<DxvkRow>,

    pub expander_row: adw::ExpanderRow
}

impl DxvkGroup {
    pub fn new(group: Group) -> Self {
        let expander_row = adw::ExpanderRow::new();

        expander_row.set_title(&group.title);
        expander_row.set_subtitle(group.subtitle.as_ref().unwrap_or(&String::new()));

        let mut version_components = Vec::new();

        for version in &group.versions {
            let component = DxvkRow::new(version.clone());

            expander_row.add_row(&component.row);

            version_components.push(component);
        }

        Self {
            group,
            version_components,
            expander_row
        }
    }

    pub fn update_states<T: ToString>(&self, runners_folder: T) {
        for component in &self.version_components {
            component.update_state(runners_folder.to_string());
        }
    }
}
