use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

pub mod size;
pub mod framerate;
pub mod window_type;

pub mod prelude {
    pub use super::Gamescope;
    pub use super::size::Size;
    pub use super::framerate::Framerate;
    pub use super::window_type::WindowType;
}

use prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gamescope {
    pub enabled: bool,
    pub game: Size,
    pub gamescope: Size,
    pub framerate: Framerate,
    pub integer_scaling: bool,
    pub fsr: bool,
    pub nis: bool,
    pub window_type: WindowType
}

impl Gamescope {
    pub fn get_command(&self) -> Option<String> {
        // https://github.com/bottlesdevs/Bottles/blob/b908311348ed1184ead23dd76f9d8af41ff24082/src/backend/wine/winecommand.py#L478
        if self.enabled {
            let mut gamescope = String::from("gamescope");

            // Set window type
            match self.window_type {
                WindowType::Borderless => gamescope += " -b",
                WindowType::Fullscreen => gamescope += " -f"
            }

            // Set game width
            if self.game.width > 0 {
                gamescope += &format!(" -w {}", self.game.width);
            }

            // Set game height
            if self.game.height > 0 {
                gamescope += &format!(" -h {}", self.game.height);
            }

            // Set gamescope width
            if self.gamescope.width > 0 {
                gamescope += &format!(" -W {}", self.gamescope.width);
            }

            // Set gamescope height
            if self.gamescope.height > 0 {
                gamescope += &format!(" -H {}", self.gamescope.height);
            }

            // Set focused framerate limit
            if self.framerate.focused > 0 {
                gamescope += &format!(" -r {}", self.framerate.focused);
            }

            // Set unfocused framerate limit
            if self.framerate.unfocused > 0 {
                gamescope += &format!(" -o {}", self.framerate.unfocused);
            }

            // Set integer scaling
            if self.integer_scaling {
                gamescope += " -n";
            }

            // Set FSR support
            if self.fsr {
                gamescope += " -U";
            }

            // Set NIS (Nvidia Image Scaling) support
            if self.nis {
                gamescope += " -Y";
            }

            Some(gamescope)
        }

        else {
            None
        }
    }
}

impl Default for Gamescope {
    fn default() -> Self {
        Self {
            enabled: false,
            game: Size::default(),
            gamescope: Size::default(),
            framerate: Framerate::default(),
            integer_scaling: true,
            fsr: false,
            nis: false,
            window_type: WindowType::default()
        }
    }
}

impl From<&JsonValue> for Gamescope {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            },

            game: match value.get("game") {
                Some(value) => Size::from(value),
                None => default.game
            },

            gamescope: match value.get("gamescope") {
                Some(value) => Size::from(value),
                None => default.gamescope
            },

            framerate: match value.get("framerate") {
                Some(value) => Framerate::from(value),
                None => default.framerate
            },

            integer_scaling: match value.get("integer_scaling") {
                Some(value) => value.as_bool().unwrap_or(default.integer_scaling),
                None => default.integer_scaling
            },

            fsr: match value.get("fsr") {
                Some(value) => value.as_bool().unwrap_or(default.fsr),
                None => default.fsr
            },

            nis: match value.get("nis") {
                Some(value) => value.as_bool().unwrap_or(default.nis),
                None => default.nis
            },

            window_type: match value.get("window_type") {
                Some(value) => WindowType::from(value),
                None => default.window_type
            }
        }
    }
}
