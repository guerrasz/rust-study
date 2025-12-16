// you can also have #[derive(PartialEq)] but it will compare all fields
pub struct Flight {
    pub time: String,
    origin: String,
    destination: String,
}

impl Flight {
    pub fn new(time: &str, origin: &str, destination: &str) -> Self {
        Self {
            time: time.to_string(),
            origin: origin.to_string(),
            destination: destination.to_string(),
        }
    }
}

pub struct Bus {
    pub time: String,
    origin: String,
    destination: String,
}

impl Bus {
    pub fn new(time: &str, origin: &str, destination: &str) -> Self {
        Self {
            time: time.to_string(),
            origin: origin.to_string(),
            destination: destination.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<Bus> for Flight {
    fn eq(&self, other: &Bus) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<Flight> for Bus {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
}

impl Eq for Flight {}
