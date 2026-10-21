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

use std::path::Path;
use std::io::Result;

/// Move files from one folder to another
pub fn move_files(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    for entry in from.as_ref().read_dir()?.flatten() {
        let source = entry.path();
        let target = to.as_ref().join(entry.file_name());

        if std::fs::rename(&source, &target).is_err() {
            if source.is_dir() {
                std::fs::create_dir_all(&target)
                    .and_then(|_| move_files(&source, &target))
                    .and_then(|_| std::fs::remove_dir_all(&source))?;
            }

            else if source.is_symlink() {
                std::fs::read_link(&source)
                    .and_then(|link_target| std::os::unix::fs::symlink(link_target, &target))
                    .and_then(|_| std::fs::remove_file(&source))?;
            }

            else {
                std::fs::copy(&source, &target)
                    .and_then(|_| std::fs::remove_file(&source))?;
            }
        }
    }

    Ok(())
}
