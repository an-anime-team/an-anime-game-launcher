use std::collections::HashMap;
use std::{fs::File, io::Read};
use std::path::Path;
use std::io::{Error, ErrorKind, Write};

use serde::{Serialize, Deserialize};

use super::consts::config_file;

pub fn get() -> Result<Config, Error> {
    match config_file() {
        Some(path) => {
            // Try to read config if the file exists
            if Path::new(&path).exists() {
                let mut file = File::open(path)?;
                let mut toml = String::new();

                file.read_to_string(&mut toml)?;

                match toml::from_str::<Config>(&toml) {
                    Ok(toml) => Ok(toml),
                    Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to decode data from toml format: {}", err.to_string())))
                }
            }

            // Otherwise create default config file
            else {
                update(Config::default())?;

                Ok(Config::default())
            }
        },
        None => Err(Error::new(ErrorKind::NotFound, format!("Failed to get config file path")))
    }
}

pub fn update(config: Config) -> Result<(), Error> {
    match config_file() {
        Some(path) => {
            let mut file = File::create(&path)?;

            match toml::to_string(&config) {
                Ok(toml) => {
                    file.write_all(&mut toml.as_bytes())?;

                    Ok(())
                },
                Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to encode data into toml format: {}", err.to_string())))
            }
        },
        None => Err(Error::new(ErrorKind::NotFound, format!("Failed to get config file path")))
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub paths: Paths,
    pub patch: Patch,
    pub wine: Wine
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Paths {
    pub game: String,
    pub patch: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Patch {
    pub hosts: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wine {
    pub prefix: String,
    pub executable: String,
    pub environment: HashMap<String, String>
}

impl Default for Wine {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            executable: String::new(),
            environment: HashMap::new()
        }
    }
}
