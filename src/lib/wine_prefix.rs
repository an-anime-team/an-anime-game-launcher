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

    fn wineboot<T: ToString>(&self, runners_folder: T, runner: super::wine::Version, command: &str) -> std::io::Result<Output> {
        let runners_folder = runners_folder.to_string();

        let wineboot = format!("{}/{}/{}", &runners_folder, runner.name, runner.files.wineboot);
        let wineserver = format!("{}/{}/{}", &runners_folder, runner.name, runner.files.wineserver);

        let mut wineboot = Command::new(wineboot);

        wineboot.env("WINEARCH", "win64")
            .env("WINESERVER", wineserver)
            .env("WINEPREFIX", &self.path)
            .arg(command);

        Ok(wineboot.output()?)
    }

    pub fn update<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wineboot(runners_folder, runner, "-u")
    }

    pub fn end<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wineboot(runners_folder, runner, "-e")
    }

    pub fn kill<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wineboot(runners_folder, runner, "-k")
    }

    pub fn restart<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wineboot(runners_folder, runner, "-r")
    }

    pub fn shutdown<T: ToString>(&self, runners_folder: T, runner: super::wine::Version) -> std::io::Result<Output> {
        self.wineboot(runners_folder, runner, "-s")
    }
}
