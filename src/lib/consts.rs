use std::time::Duration;

static mut LAUNCHER_DIR: Option<Option<String>> = None;
static mut CONFIG_FILE: Option<Option<String>> = None;

/// Timeout used by `anime_game_core::telemetry::is_disabled` to check acessibility of telemetry servers
pub const TELEMETRY_CHECK_TIMEOUT: Option<Duration> = Some(Duration::from_secs(3));

/// Timeout used by `anime_game_core::linux_patch::Patch::try_fetch` to fetch patch info
pub const PATCH_FETCHING_TIMEOUT: Option<Duration> = Some(Duration::from_secs(5));

pub fn launcher_dir() -> Option<String> {
    unsafe {
        match &LAUNCHER_DIR {
            Some(value) => value.clone(),
            None => {
                let value = match dirs::data_dir() {
                    Some(dir) => Some(format!("{}/anime-game-launcher", dir.to_string_lossy())),
                    None => None
                };

                LAUNCHER_DIR = Some(value.clone());
    
                value
            }
        }
    }
}

pub fn config_file() -> Option<String> {
    unsafe {
        match &CONFIG_FILE {
            Some(value) => value.clone(),
            None => {
                let value = match launcher_dir() {
                    Some(dir) => Some(format!("{}/config.json", dir)),
                    None => None
                };

                CONFIG_FILE = Some(value.clone());
    
                value
            }
        }
    }
}
