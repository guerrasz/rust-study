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

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}
