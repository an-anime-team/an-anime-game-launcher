use serde::Serialize;

use super::FpsUnlockerConfig;

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct ConfigSchema {
    pub DllList: Vec<String>,
    pub Priority: u64,
    pub MonitorNum: u64,
    pub CustomResY: u64,
    pub CustomResX: u64,
    pub FPSTarget: u64,
    pub UsePowerSave: bool,
    pub StartMinimized: bool,
    pub IsExclusiveFullscreen: bool,
    pub UseCustomRes: bool,
    pub Fullscreen: bool,
    pub PopupWindow: bool,
    pub AutoClose: bool,
    pub AutoDisableVSync: bool,
    pub AutoStart: bool,
    pub GamePath: Option<String>
}

impl Default for ConfigSchema {
    fn default() -> Self {
        Self {
            DllList: vec![],
            Priority: 3,
            MonitorNum: 1,
            CustomResY: 1080,
            CustomResX: 1920,
            FPSTarget: 120,
            UsePowerSave: false,
            IsExclusiveFullscreen: false,
            UseCustomRes: false,
            Fullscreen: false,
            PopupWindow: false,
            AutoDisableVSync: true,
            GamePath: None,

            // Launcher-specific settings
            AutoStart: true,
            AutoClose: true,
            StartMinimized: true
        }
    }
}

impl ConfigSchema {
    pub fn from_config(config: FpsUnlockerConfig) -> Self {
        Self {
            FPSTarget: config.fps,
            UsePowerSave: config.power_saving,
            PopupWindow: config.window_mode == 1,
            Fullscreen: config.window_mode == 2,
            MonitorNum: config.monitor,
            Priority: config.priority,

            ..Self::default()
        }
    }

    pub fn json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
