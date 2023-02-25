use anime_launcher_sdk::anime_game_core::installer::downloader::Downloader;
use anime_launcher_sdk::anime_game_core::curl::fetch;

#[derive(Debug, Clone)]
pub struct Background {
    pub uri: String,
    pub hash: String
}

pub fn get_uri() -> String {
    let uri = concat!("https://sdk-os-static.", "ho", "yo", "verse", ".com/hk4e_global/mdk/launcher/api/content?filter_adv=true&key=gcStgarh&launcher_id=10&language=");

    uri.to_owned() + crate::i18n::get_lang()
}

#[cached::proc_macro::cached(result)]
pub fn get_background_info() -> anyhow::Result<Background> {
    let json = serde_json::from_slice::<serde_json::Value>(&fetch(get_uri(), None)?.get_body()?)?;

    let uri = match json["data"]["adv"]["background"].as_str() {
        Some(uri) => uri.to_owned(),
        None => anyhow::bail!("Failed to get background picture uri")
    };

    // This API field contains wrong md5 hash, but file's name
    // from the uri above actually contains correct one, so
    // I parse and use it few lines below

    /*let hash = match json["data"]["adv"]["bg_checksum"].as_str() {
        Some(uri) => uri.to_owned(),
        None => anyhow::bail!("Failed to get background picture checksum")
    };*/

    let hash = uri.split('/').last().unwrap_or_default().split('_').next().unwrap_or_default().to_owned();

    Ok(Background {
        uri,
        hash
    })
}

pub fn download_background() -> anyhow::Result<()> {
    tracing::debug!("Downloading background picture");

    let info = get_background_info()?;

    if crate::BACKGROUND_FILE.exists() {
        let hash = md5::compute(std::fs::read(crate::BACKGROUND_FILE.as_path())?);

        if format!("{:x}", hash).to_lowercase() == info.hash {
            tracing::debug!("Background picture is already downloaded. Skipping");

            return Ok(());
        }
    }

    let mut downloader = Downloader::new(info.uri)?;

    if let Err(err) = downloader.download_to(crate::BACKGROUND_FILE.as_path(), |_, _| {}) {
        let err: std::io::Error = err.into();

        anyhow::bail!(err);
    }

    Ok(())
}
