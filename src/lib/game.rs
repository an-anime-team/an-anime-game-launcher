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

    pub fn get_args(&self, bash_command: &str) -> Vec<String> {
        match self {
            Terminal::GnomeTerminal => vec![
                String::from("--"),
                String::from("bash"),
                String::from("-c"),
                format!("{} && bash", bash_command)
            ],
            Terminal::Konsole | Terminal::Xfce4Terminal => vec![
                String::from("--hold"),
                String::from("-e"),
                format!("\"bash -c '{} && bash'\"", bash_command)
            ]
        }
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
/// 
/// If `debug = true`, then the game will be run in the new terminal window
pub fn run(debug: bool) -> Result<(), Error> {
    let config = config::get()?;

    if !Path::new(&config.game.path).exists() {
        return Err(Error::new(ErrorKind::Other, "Game is not installed"));
    }

    let wine_executable = match config.try_get_wine_executable() {
        Some(path) => path,
        None => return Err(Error::new(ErrorKind::Other, "Couldn't find wine executable"))
    };

    let mut command = Command::new(wine_executable);

    if debug {
        // Is not supported now because new spawned terminal needs
        // to have cwd and env variables specified directly
        // which is kinda difficult
        todo!();

        match try_get_terminal() {
            Some(terminal) => {
                command = Command::new(terminal.get_command());

                command.args(terminal.get_args("launcher.bat"));
            },
            None => return Err(Error::new(ErrorKind::Other, "Couldn't find terminal application"))
        }
    }

    else {
        command.arg("launcher.bat");
    }

    command.env("WINEPREFIX", &config.game.wine.prefix);
    command.envs(config.get_wine_sync_env_vars());

    command.envs(config.game.environment)
        .current_dir(config.game.path)
        .spawn()?;
    
    Ok(())
}
