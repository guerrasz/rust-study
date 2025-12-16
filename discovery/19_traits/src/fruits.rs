use std::fmt::{Debug, Display, Formatter, Result};

pub enum AppleType {
    Red,
    Green,
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Green => write!(f, "Green Apple"),
            Self::Red => write!(f, "Red Apple"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Green => write!(f, "AppleType::Green"),
            Self::Red => write!(f, "AppleType::Red"),
        }
    }
}

pub struct Apple {
    pub kind: AppleType,
    pub price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} 🍎 for ${:.2}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Apple ::: [Kind: {:?}, Price: {:?}]",
            self.kind, self.price
        )
    }
}
