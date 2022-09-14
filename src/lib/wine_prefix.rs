use std::path::Path;
use std::process::{Command, Output};

#[derive(Debug, Clone)]
pub struct WinePrefix {
    pub path: String
}

impl WinePrefix {
    pub fn new<T: ToString>(path: T) -> Self {
        Self { path: path.to_string() }
    }

    pub fn exists(&self) -> bool {
        Self::exists_in(&self.path)
    }

    pub fn exists_in<T: ToString>(path: T) -> bool {
        Path::new(&format!("{}/drive_c", path.to_string())).exists()
    }

    fn wine<T: ToString>(&self, wine_binary: T, command: &str) -> std::io::Result<Output> {
        let mut wine_command = Command::new(wine_binary.to_string());

        wine_command.env("WINEARCH", "win64")
            .env("WINEPREFIX", &self.path)
            .arg(command);

        Ok(wine_command.output()?)
    }

    pub fn update<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.update_with(format!("{}/{}/{}", runners_folder.to_string(), runner.name, runner.files.wine64))
    }

    pub fn update_with<T: ToString>(&self, wine_binary: T) -> std::io::Result<Output> {
        self.wine(wine_binary, "-u")
    }
}
