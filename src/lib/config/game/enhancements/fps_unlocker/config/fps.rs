use gtk4 as gtk;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fps {
    /// 90
    Ninety,

    /// 100
    Hundred,

    /// 120
    HundredTwenty,

    /// 144
    HundredFourtyFour,

    /// 200
    TwoHundred,

    Custom(u64)
}

impl Fps {
    pub fn list() -> Vec<Self> {
        vec![
            Self::Ninety,
            Self::Hundred,
            Self::HundredTwenty,
            Self::HundredFourtyFour,
            Self::TwoHundred
        ]
    }

    pub fn get_model() -> gtk::StringList {
        let model = gtk::StringList::new(&[]);

        model.append("Custom");

        for res in Self::list() {
            model.append(&res.to_num().to_string());
        }

        model
    }

    pub fn from_num(fps: u64) -> Self {
        match fps {
            90 => Self::Ninety,
            100 => Self::Hundred,
            120 => Self::HundredTwenty,
            144 => Self::HundredFourtyFour,
            200 => Self::TwoHundred,
            num => Self::Custom(num)
        }
    }

    pub fn to_num(&self) -> u64 {
        match self {
            Fps::Ninety => 90,
            Fps::Hundred => 100,
            Fps::HundredTwenty => 120,
            Fps::HundredFourtyFour => 144,
            Fps::TwoHundred => 200,
            Fps::Custom(num) => *num
        }
    }
}
