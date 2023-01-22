pub mod list;
pub mod group;
pub mod version;

pub use list::*;
pub use group::*;
pub use version::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListPattern {
    pub download_folder: String,
    pub groups: Vec<ComponentsListGroup>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListGroup {
    pub title: String,
    pub versions: Vec<ComponentsListVersion>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListVersion {
    pub title: String,
    pub url: String
}
