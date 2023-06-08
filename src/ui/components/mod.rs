pub mod group;
pub mod list;
pub mod progress_bar;
pub mod version;

pub use group::*;
pub use list::*;
pub use progress_bar::*;
pub use version::*;

use anime_launcher_sdk::components::*;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListPattern {
    pub download_folder: PathBuf,
    pub groups: Vec<ComponentsListGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListGroup {
    pub title: String,
    pub versions: Vec<ComponentsListVersion>,
}

impl From<wine::Group> for ComponentsListGroup {
    #[inline]
    fn from(group: wine::Group) -> Self {
        Self {
            title: group.title,
            versions: group
                .versions
                .into_iter()
                .map(|version| version.into())
                .collect(),
        }
    }
}

impl From<dxvk::Group> for ComponentsListGroup {
    #[inline]
    fn from(group: dxvk::Group) -> Self {
        Self {
            title: group.title,
            versions: group
                .versions
                .into_iter()
                .map(|version| version.into())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListVersion {
    pub name: String,
    pub title: String,
    pub uri: String,
    pub recommended: bool,
}

impl From<wine::Version> for ComponentsListVersion {
    #[inline]
    fn from(version: wine::Version) -> Self {
        Self {
            recommended: match version.version_features() {
                Some(features) => features.recommended,
                None => true,
            },

            name: version.name,
            title: version.title,
            uri: version.uri,
        }
    }
}

impl From<dxvk::Version> for ComponentsListVersion {
    #[inline]
    fn from(version: dxvk::Version) -> Self {
        Self {
            recommended: match version.version_features() {
                Some(features) => features.recommended,
                None => true,
            },

            name: version.name,
            title: version.title,
            uri: version.uri,
        }
    }
}
