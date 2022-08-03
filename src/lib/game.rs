use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

use anime_game_core::telemetry;

use super::consts;
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
pub fn run(debug: bool) -> std::io::Result<()> {
    let config = config::get()?;

    if !Path::new(&config.game.path).exists() {
        return Err(Error::new(ErrorKind::Other, "Game is not installed"));
    }

    let wine_executable = match config.try_get_wine_executable() {
        Some(path) => path,
        None => return Err(Error::new(ErrorKind::Other, "Couldn't find wine executable"))
    };

    // Check telemetry servers

    if let Some(server) = telemetry::is_disabled(consts::TELEMETRY_CHECK_TIMEOUT) {
        return Err(Error::new(ErrorKind::Other, format!("Telemetry server is not disabled: {server}")));
    }

    // Prepare bash -c '<command>'

    let mut bash_chain = String::new();

    if config.game.enhancements.gamemode {
        bash_chain += "gamemoderun ";
    }

    bash_chain += &format!("'{}' ", wine_executable);

    if debug {
        // Is not supported now because new spawned terminal needs
        // to have cwd and env variables specified directly
        // which is kinda difficult
        todo!();

        /*match try_get_terminal() {
            Some(terminal) => {
                command = Command::new(terminal.get_command());

                command.args(terminal.get_args("launcher.bat"));
            },
            None => return Err(Error::new(ErrorKind::Other, "Couldn't find terminal application"))
        }*/
    }

    else {
        bash_chain += "launcher.bat";
    }

    let bash_chain = match config.game.command {
        Some(command) => command.replace("%command%", &bash_chain),
        None => bash_chain
    };

    let mut command = Command::new("bash");

    command.arg("-c");
    command.arg(&bash_chain);

    // Setup environment

    command.env("WINEARCH", "win64");
    command.env("WINEPREFIX", &config.game.wine.prefix);

    // Add DXVK_ASYNC=1 for dxvk-async builds automatically
    if let Some(dxvk) = config.game.dxvk.selected {
        if dxvk.contains("async") {
            command.env("DXVK_ASYNC", "1");
        }
    }

    command.envs(config.game.wine.sync.get_env_vars());
    command.envs(config.game.enhancements.hud.get_env_vars());
    command.envs(config.game.enhancements.fsr.get_env_vars());
    command.envs(config.game.wine.language.get_env_vars());

    command.envs(config.game.environment);

    // Run command

    println!("Running command: bash -c \"{}\"", bash_chain);

    command.current_dir(config.game.path).spawn()?;
    
    Ok(())
}
