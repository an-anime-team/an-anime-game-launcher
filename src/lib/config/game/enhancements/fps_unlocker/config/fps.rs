#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fps {
    /// 90
    Ninety,

    /// 120
    HundredTwenty,

    /// 144
    HundredFourtyFour,

    /// 165
    HundredSixtyFive,

    /// 180
    HundredEighty,

    /// 200
    TwoHundred,

    /// 240
    TwoHundredFourty,

    Custom(u64)
}

impl Fps {
    pub fn list() -> Vec<Self> {
        vec![
            Self::Ninety,
            Self::HundredTwenty,
            Self::HundredFourtyFour,
            Self::HundredSixtyFive,
            Self::HundredEighty,
            Self::TwoHundred,
            Self::TwoHundredFourty
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
            120 => Self::HundredTwenty,
            144 => Self::HundredFourtyFour,
            165 => Self::HundredSixtyFive,
            180 => Self::HundredEighty,
            200 => Self::TwoHundred,
            240 => Self::TwoHundredFourty,
            num => Self::Custom(num)
        }
    }

    pub fn to_num(&self) -> u64 {
        match self {
            Self::Ninety            => 90,
            Self::HundredTwenty     => 120,
            Self::HundredFourtyFour => 144,
            Self::HundredSixtyFive  => 165,
            Self::HundredEighty     => 180,
            Self::TwoHundred        => 200,
            Self::TwoHundredFourty  => 240,
            Self::Custom(num)  => *num
        }
    }
}
