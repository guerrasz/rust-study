// it's possile to also use: #[derive(Clone)]
pub struct Appointment {
    pub doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    pub fn new(name: &str, start: &str, end: &str) -> Self {
        Self {
            doctor: name.to_string(),
            start_time: start.to_string(),
            end_time: end.to_string(),
        }
    }
}

impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}
