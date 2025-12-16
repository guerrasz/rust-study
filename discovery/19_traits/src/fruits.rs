use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

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

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Goodbye my sweet apple"),
            Err(error) => println!("Could not delete file, error: {}", error),
        }
    }
}
