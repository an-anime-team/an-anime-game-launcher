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
        Path::new(&format!("{}/drive_c", self.path)).exists()
    }

    fn wine<T: ToString>(&self, runners_folder: T, runner: super::wine::Version, command: &str) -> std::io::Result<Output> {
        let runners_folder = runners_folder.to_string();

        let wine = format!("{}/{}/{}", &runners_folder, runner.name, runner.files.wine64);
        let wineserver = format!("{}/{}/{}", &runners_folder, runner.name, runner.files.wineserver);

        let mut wineboot = Command::new(wine);

        wineboot.env("WINEARCH", "win64")
            .env("WINESERVER", wineserver)
            .env("WINEPREFIX", &self.path)
            .arg(command);

        Ok(wineboot.output()?)
    }

    pub fn update<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wine(runners_folder, runner, "-u")
    }

    pub fn end<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wine(runners_folder, runner, "-e")
    }

    pub fn kill<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wine(runners_folder, runner, "-k")
    }

    pub fn restart<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wine(runners_folder, runner, "-r")
    }

    pub fn shutdown<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wine(runners_folder, runner, "-s")
    }
}
