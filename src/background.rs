use std::process::Command;

use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;
use anime_launcher_sdk::anime_game_core::minreq;

use md5::{Md5, Digest};

#[derive(Debug, Clone)]
pub struct Background {
    pub uri: String,
    pub hash: String
}

pub fn get_uri() -> String {
    let lang = crate::i18n::get_lang();

    if lang.language == unic_langid::langid!("zh-cn").language {
        concat!("https://hyp-api.", "mi", "ho", "yo", ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=jGHBHlcOq1").to_owned()
    }

    else {
        let uri = concat!("https://sg-hyp-api.", "ho", "yo", "verse", ".com/hyp/hyp-connect/api/getAllGameBasicInfo?launcher_id=VYTpXlbWo8&language=");

        uri.to_owned() + &crate::i18n::format_lang(&lang)
    }
}

#[cached::proc_macro::cached(result)]
pub fn get_background_info() -> anyhow::Result<Background> {
    let json = serde_json::from_slice::<serde_json::Value>(minreq::get(get_uri()).send()?.as_bytes())?;

    let uri = json["data"]["game_info_list"].as_array()
        .ok_or_else(|| anyhow::anyhow!("Failed to list games in the backgrounds API"))?
        .iter()
        .find(|game| game["game"]["biz"].as_str() == Some("hk4e_global"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find the game in the backgrounds API"))?["backgrounds"]
        .as_array()
        .and_then(|backgrounds| backgrounds.iter().next())
        .and_then(|background| background["background"]["url"].as_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get background picture url"))?
        .to_string();

    let hash = uri.split('/')
        .last()
        .unwrap_or_default()
        .split('_')
        .next()
        .unwrap_or_default()
        .to_owned();

    Ok(Background {
        uri,
        hash
    })
}

pub fn download_background() -> anyhow::Result<()> {
    tracing::debug!("Downloading background picture");

    let info = get_background_info()?;

    if crate::BACKGROUND_FILE.exists() {
        let hash = Md5::digest(std::fs::read(crate::BACKGROUND_FILE.as_path())?);

        if format!("{:x}", hash).to_lowercase() == info.hash {
            tracing::debug!("Background picture is already downloaded. Skipping");

            return Ok(());
        }
    }

    let mut downloader = Downloader::new(&info.uri)?;

    downloader.continue_downloading = false;

    if let Err(err) = downloader.download(crate::BACKGROUND_FILE.as_path(), |_, _| {}) {
        anyhow::bail!(err);
    }

    // Workaround for GTK weakness
    if info.uri.ends_with(".webp") {
        Command::new("dwebp")
            .arg(crate::BACKGROUND_FILE.as_path())
            .arg("-o")
            .arg(crate::BACKGROUND_PRIMARY_FILE.as_path())
            .spawn()?
            .wait_with_output()?;
    }

    else {
        std::fs::copy(crate::BACKGROUND_FILE.as_path(), crate::BACKGROUND_PRIMARY_FILE.as_path())?;
    }

    Ok(())
}
