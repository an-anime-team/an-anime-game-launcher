use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

use super::config;

pub fn run() -> Result<(), Error> {
    let config = config::get()?;

    if Path::new(&config.paths.game).exists() {
        return Err(Error::new(ErrorKind::Other, "Game is not installed"));
    }

    Command::new(config.wine.executable)
        .env("WINEPREFIX", &config.wine.prefix)
        .envs(config.wine.environment)
        .current_dir(config.paths.game)
        .arg("launcher.bat")
        .output()?;
    
    Ok(())
}
