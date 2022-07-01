use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

use super::config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terminal {
    GnomeTerminal,
    Konsole,
    Xfce4Terminal
}

impl Terminal {
    pub fn get_command(&self) -> &str {
        match self {
            Terminal::GnomeTerminal => "gnome-terminal",
            Terminal::Konsole => "konsole",
            Terminal::Xfce4Terminal => "xfce4-terminal"
        }
    }

    pub fn iter() -> impl Iterator<Item = Terminal> {
        [
            Terminal::GnomeTerminal,
            Terminal::Konsole,
            Terminal::Xfce4Terminal
        ].into_iter()
    }
}

/// Try to get GUI terminal installed in system
pub fn try_get_terminal() -> Option<Terminal> {
    for terminal in Terminal::iter() {
        if let Ok(output) = Command::new(terminal.get_command()).output() {
            if output.status.success() {
                return Some(terminal);
            }
        }
    }

    None
}

/// Try to run the game
pub fn run() -> Result<(), Error> {
    let config = config::get()?;

    if !Path::new(&config.game.path).exists() {
        return Err(Error::new(ErrorKind::Other, "Game is not installed"));
    }

    let wine_executable = match config.try_get_wine_executable() {
        Some(path) => path,
        None => return Err(Error::new(ErrorKind::Other, "Couldn't find wine executable"))
    };

    let mut command = Command::new(wine_executable);

    command.env("WINEPREFIX", &config.game.wine.prefix);
    command.envs(config.get_wine_sync_env_vars());

    command.envs(config.game.environment)
        .current_dir(config.game.path)
        .arg("launcher.bat")
        .spawn()?;
    
    Ok(())
}
