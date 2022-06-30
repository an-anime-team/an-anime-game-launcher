static mut LAUNCHER_DIR: Option<Option<String>> = None;
static mut CONFIG_FILE: Option<Option<String>> = None;

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
                    Some(dir) => Some(format!("{}/config.toml", dir)),
                    None => None
                };

                CONFIG_FILE = Some(value.clone());
    
                value
            }
        }
    }
}
