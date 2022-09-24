#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resolution {
    // qHD; 960x540
    MiniHD,

    // 1280x720
    HD,

    // 1920x1080
    FullHD,

    // 2560x1440
    QuadHD,

    // 3840x2160
    UltraHD,

    Custom(u64, u64)
}

impl Resolution {
    pub fn list() -> Vec<Self> {
        vec![
            Self::MiniHD,
            Self::HD,
            Self::FullHD,
            Self::QuadHD,
            Self::UltraHD
        ]
    }

    pub fn get_model() -> gtk::StringList {
        let model = gtk::StringList::new(&[]);

        model.append("Custom");

        for res in Self::list() {
            model.append(&res.to_string());
        }

        model
    }

    pub fn from_pair(width: u64, height: u64) -> Self {
        for res in Self::list() {
            let pair = res.get_pair();

            if pair.0 == width && pair.1 == height {
                return res;
            }
        }

        Self::Custom(width, height)
    }

    pub fn get_pair(&self) -> (u64, u64) {
        match self {
            Self::MiniHD  => (960, 540),
            Self::HD      => (1280, 720),
            Self::FullHD  => (1920, 1080),
            Self::QuadHD  => (2560, 1440),
            Self::UltraHD => (3840, 2160),

            Self::Custom(w, h) => (*w, *h)
        }
    }

    pub fn to_string(&self) -> String {
        let (w, h) = self.get_pair();

        format!("{w}x{h}")
    }
}
