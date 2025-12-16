// can be done #[derive(PartialEq)]
pub enum Musician {
    Singer(String),
    Band(u32),
}

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Singer(name) => match other {
                Self::Singer(other_name) => name == other_name,
                Self::Band(_) => false,
            },
            Self::Band(num) => match other {
                Self::Band(other_num) => num == other_num,
                Self::Singer(_) => false,
            },
        }
    }
}
