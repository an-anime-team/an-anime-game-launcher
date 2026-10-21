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
use std::process::Command;

use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;
use anime_launcher_sdk::is_available;
use anime_launcher_sdk::anime_game_core::minreq;

use anyhow::{Context, anyhow};
use md5::{Digest, Md5};

use crate::STARTUP_LANG;

pub fn download_background(with_video: bool, _index: u8) -> anyhow::Result<()> {
    tracing::debug!("Downloading background picture");

    let backgrounds = get_background_info_multiple()?;
    #[allow(unused_parens, reason = "Clarity in the `find` condition")]
    let info = backgrounds
        .iter()
        .find(|bginfo| (!with_video || matches!(bginfo, BackgroundSpec::Video { .. })))
        .or(backgrounds.first())
        .ok_or(anyhow!(
            "Failed to get background information: no backgrounds in the API"
        ))?;

    let regenerate_image = info.download(with_video)?;

    if regenerate_image {
        if gtk_webp_image_supported() {
            std::fs::copy(&*crate::BACKGROUND_FILE, &*crate::PROCESSED_BACKGROUND_FILE)
                .context("Copying background file")?;
            if matches!(info, BackgroundSpec::Video { .. }) {
                std::fs::copy(
                    &*crate::BACKGROUND_OVERLAY_FILE,
                    &*crate::PROCESSED_BACKGROUND_OVERLAY_FILE
                )
                .context("Copying background overlay file")?;
            }
        } else {
            tracing::info!("WebP GDK Pixbuf Loader is not installed, converting images to PNG");
            info.convert_and_copy()?;
        }

        if matches!(info, BackgroundSpec::Normal { .. }) {
            // Remove the overlay and video file if it's normal variant
            // Ignore error, if file is already missing for example
            let _ = std::fs::remove_file(&*crate::PROCESSED_BACKGROUND_OVERLAY_FILE);
            let _ = std::fs::remove_file(&*crate::BACKGROUND_VIDEO_FILE);
        }
    } else {
        tracing::debug!("Not re-generating the background image, already latest")
    }

    Ok(())
}

#[cached::proc_macro::cached]
pub fn get_background_info_multiple() -> anyhow::Result<Vec<BackgroundSpec>> {
    let json = serde_json::from_slice::<serde_json::Value>(
        minreq::get(get_uri()).with_timeout(15).send()?.as_bytes()
    )?;

    BackgroundSpec::from_json_all(&json)
}

#[cached::proc_macro::cached]
pub fn get_background_info(index: u8) -> anyhow::Result<BackgroundSpec> {
    let json = serde_json::from_slice::<serde_json::Value>(
        minreq::get(get_uri()).with_timeout(15).send()?.as_bytes()
    )?;

    BackgroundSpec::from_json_single(&json, index)
}

pub fn get_uri() -> String {
    if STARTUP_LANG.language == "zh" {
        concat!(
            "https://hyp-api.",
            "mi",
            "ho",
            "yo",
            ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=jGHBHlcOq1"
        )
        .to_owned()
    } else {
        let uri = concat!(
            "https://sg-hyp-api.",
            "ho",
            "yo",
            "verse",
            ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=VYTpXlbWo8&language="
        );

        uri.to_owned() + &crate::i18n::format_lang(&STARTUP_LANG)
    }
}

#[derive(Debug, Clone)]
pub enum BackgroundSpec {
    Normal {
        background: Background
    },
    Video {
        background: Background,
        video: Background,
        overlay: Background
    }
}

impl BackgroundSpec {
    fn from_json_single(value: &serde_json::Value, index: u8) -> anyhow::Result<Self> {
        let backgrounds_json = Self::backgrounds_json_from_value(value)?;
        Self::from_json_value(
            backgrounds_json
                .get(index as usize)
                .or_else(|| backgrounds_json.first())
                .ok_or_else(|| anyhow::anyhow!("The API did not provide any backgrounds"))?
        )
    }

    fn from_json_all(value: &serde_json::Value) -> anyhow::Result<Vec<Self>> {
        let backgrounds_json = Self::backgrounds_json_from_value(value)?;

        backgrounds_json.iter().map(Self::from_json_value).collect()
    }

    fn backgrounds_json_from_value(
        value: &serde_json::Value
    ) -> anyhow::Result<&Vec<serde_json::Value>> {
        value["data"]["game_info_list"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Failed to list games in the backgrounds API"))?
                .iter()
                .find(|game| match game["game"]["biz"].as_str() {
                    Some(biz) => biz.starts_with("hk4e_"),
                    _ => false
                })
                .ok_or_else(|| anyhow::anyhow!("Failed to find the game in the backgrounds API"))?
                ["backgrounds"]
                .as_array()
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Failed to parse backgrounds API: `backgrounds` is not an array"
                    )
                })
    }

    fn from_json_value(value: &serde_json::Value) -> anyhow::Result<Self> {
        let background_uri = get_img_uri_from_json_value(Some(value), "background")?;
        let background = Background::from_uri(background_uri);

        if value["type"].as_str() == Some("BACKGROUND_TYPE_VIDEO") {
            let video_uri = get_img_uri_from_json_value(Some(value), "video")?;
            let video = Background::from_uri(video_uri);

            let overlay_uri = get_img_uri_from_json_value(Some(value), "theme")?;
            let overlay = Background::from_uri(overlay_uri);

            Ok(Self::Video {
                background,
                video,
                overlay
            })
        } else {
            Ok(Self::Normal {
                background
            })
        }
    }

    fn background(&self) -> &Background {
        match self {
            Self::Normal {
                background
            }
            | Self::Video {
                background, ..
            } => background
        }
    }

    /// Returns true if the background needs to be re-generated
    fn download(&self, with_video: bool) -> anyhow::Result<bool> {
        let mut regenerate_image = false;

        regenerate_image |= self.background().download(&crate::BACKGROUND_FILE)?;

        if let Self::Video {
            video,
            overlay,
            ..
        } = self
        {
            regenerate_image |= overlay.download(&crate::BACKGROUND_OVERLAY_FILE)?;
            if with_video {
                regenerate_image |= video.download(&crate::BACKGROUND_VIDEO_FILE)?;
            }
        }

        Ok(regenerate_image)
    }

    fn convert_and_copy(&self) -> anyhow::Result<()> {
        finalize_file(
            self.background(),
            &crate::BACKGROUND_FILE,
            &crate::PROCESSED_BACKGROUND_FILE
        )?;
        if let Self::Video {
            overlay, ..
        } = self
        {
            finalize_file(
                overlay,
                &crate::BACKGROUND_OVERLAY_FILE,
                &crate::PROCESSED_BACKGROUND_OVERLAY_FILE
            )?;
        }
        Ok(())
    }
}

fn finalize_file(bg_info: &Background, from: &Path, to: &Path) -> anyhow::Result<()> {
    if bg_info.uri.ends_with(".webp") {
        convert_image(from, to).context(format!("Converting image {to:?}"))?;
    }

    // If it failed to re-code the file - just copy it
    // Will happen with HSR because devs apparently named
    // their background image ".webp" while it's JPEG
    if !to.exists() {
        std::fs::copy(from, to).context(format!("Copying {to:?}"))?;
    }

    Ok(())
}

fn convert_image(from: &Path, to: &Path) -> anyhow::Result<()> {
    if is_available("dwebp") {
        Command::new("dwebp")
            .arg(from)
            .arg("-o")
            .arg(to)
            .spawn()?
            .wait()?;
    } else if is_available("magick") {
        Command::new("magick")
            .arg(from)
            .arg(format!("PNG:{}", to.display()))
            .spawn()?
            .wait()?;
    } else {
        tracing::warn!("Could not find `dwebp` or `magick` to convert the image file.");
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Background {
    pub uri: String,
    pub hash: String
}

impl Background {
    fn from_uri(uri: String) -> Self {
        let hash = get_img_hash_from_uri(&uri);
        Self {
            uri,
            hash
        }
    }

    /// Return true if the background needs to be re-generated
    fn download(&self, path: &Path) -> anyhow::Result<bool> {
        if !check_img_file(path, &self.hash)? {
            download_img_file(path, &self.uri)?;
            return Ok(true);
        }
        Ok(false)
    }
}

/// Returns true if image exists and is correct
fn check_img_file(path: &Path, expected_hash: &str) -> anyhow::Result<bool> {
    if path.exists() {
        let mut hash = [0; 16];

        hash.copy_from_slice(&Md5::digest(std::fs::read(path)?));

        let hash = u128::from_le_bytes(hash);

        if format!("{hash:x}").eq_ignore_ascii_case(expected_hash) {
            tracing::debug!("Background picture {path:?} already downloaded. Skipping");

            return Ok(true);
        }
    }

    Ok(false)
}

fn get_img_uri_from_json_value(
    backgrounds_info: Option<&serde_json::Value>,
    key: &str
) -> anyhow::Result<String> {
    Ok(backgrounds_info
        .and_then(|background| background[key]["url"].as_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get background picture url"))?
        .to_string())
}

fn get_img_hash_from_uri(uri: &str) -> String {
    uri.split('/')
        .next_back()
        .unwrap_or_default()
        .split('_')
        .next()
        .unwrap_or_default()
        .to_owned()
}

#[cached::proc_macro::once()]
fn gtk_webp_image_supported() -> bool {
    let supported_pixbuf_formats = gtk::gdk_pixbuf::Pixbuf::formats();
    supported_pixbuf_formats.into_iter().any(|format| {
        format
            .name()
            .map(|name| name.eq_ignore_ascii_case("webp"))
            .unwrap_or(false)
            || format
                .extensions()
                .iter()
                .any(|ext| ext.eq_ignore_ascii_case("webp"))
    })
}

fn download_img_file(path: &Path, uri: &str) -> anyhow::Result<()> {
    let mut downloader = Downloader::new(uri)?;

    downloader.continue_downloading = false;

    if let Err(err) = downloader.download(path, |_, _| {}) {
        anyhow::bail!(err);
    }

    Ok(())
}
