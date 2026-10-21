// SPDX-License-Identifier: GPL-3.0-or-later
//
// an-anime-game-launcher
// Copyright (C) 2021 - 2026  Nikita Podvirnyi <krypt0nn@dawn.wine>
// Copyright (C) 2025 - 2026  @JohnTheCoolingFan <ivan8215145640@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::path::PathBuf;

pub mod list;
pub mod group;
pub mod version;
pub mod progress_bar;

pub use list::*;
pub use group::*;
pub use version::*;
pub use progress_bar::*;

use anime_launcher_sdk::components::*;

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
    #[inline]
    fn from(group: wine::Group) -> Self {
        Self {
            title: group.title,
            versions: group.versions.into_iter().map(|version| version.into()).collect()
        }
    }
}

impl From<dxvk::Group> for ComponentsListGroup {
    #[inline]
    fn from(group: dxvk::Group) -> Self {
        Self {
            title: group.title,
            versions: group.versions.into_iter().map(|version| version.into()).collect()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentsListVersion {
    pub name: String,
    pub title: String,
    pub uri: String,
    pub format: Option<String>,
    pub recommended: bool
}

impl From<wine::Version> for ComponentsListVersion {
    #[inline]
    fn from(version: wine::Version) -> Self {
        Self {
            recommended: match version.version_features() {
                Some(features) => features.recommended,
                None => true
            },

            name: version.name,
            title: version.title,
            uri: version.uri,
            format: version.format
        }
    }
}

impl From<dxvk::Version> for ComponentsListVersion {
    #[inline]
    fn from(version: dxvk::Version) -> Self {
        Self {
            recommended: match version.version_features() {
                Some(features) => features.recommended,
                None => true
            },

            name: version.name,
            title: version.title,
            uri: version.uri,
            format: version.format
        }
    }
}
