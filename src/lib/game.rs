use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

use super::config;

pub fn run() -> Result<(), Error> {
    let config = config::get()?;

    if !Path::new(&config.game.path).exists() {
        return Err(Error::new(ErrorKind::Other, "Game is not installed"));
    }

    let wine_executable = match config.try_get_wine_executable() {
        Some(path) => path,
        None => return Err(Error::new(ErrorKind::Other, "Couldn't find wine executable"))
    };

    Command::new(wine_executable)
        .env("WINEPREFIX", config.game.wine.prefix)
        .envs(config.game.environment)
        .current_dir(config.game.path)
        .arg("launcher.bat")
        .spawn()?;
    
    Ok(())
}
