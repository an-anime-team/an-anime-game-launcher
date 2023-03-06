pub mod list;
pub mod group;
pub mod version;
pub mod progress_bar;

pub use list::*;
pub use group::*;
pub use version::*;
pub use progress_bar::*;

use anime_launcher_sdk::components::*;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListPattern {
    pub download_folder: PathBuf,
    pub groups: Vec<ComponentsListGroup>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListGroup {
    pub title: String,
    pub versions: Vec<ComponentsListVersion>
}

impl From<wine::Group> for ComponentsListGroup {
    fn from(group: wine::Group) -> Self {
        Self {
            title: group.title,
            versions: group.versions.into_iter().map(|version| version.into()).collect()
        }
    }
}

impl From<dxvk::Group> for ComponentsListGroup {
    fn from(group: dxvk::Group) -> Self {
        Self {
            title: group.name,
            versions: group.versions.into_iter().map(|version| version.into()).collect()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListVersion {
    pub name: String,
    pub title: String,
    pub uri: String,
    pub recommended: bool
}

impl From<wine::Version> for ComponentsListVersion {
    fn from(version: wine::Version) -> Self {
        Self {
            name: version.name,
            title: version.title,
            uri: version.uri,
            recommended: true // FIXME
        }
    }
}

impl From<dxvk::Version> for ComponentsListVersion {
    fn from(version: dxvk::Version) -> Self {
        Self {
            name: version.name.clone(),
            title: version.name,
            uri: version.uri,
            recommended: true // FIXME
        }
    }
}
