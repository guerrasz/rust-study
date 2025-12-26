#![allow(unused)]
use crate::attractions::{MovieTheater, TicketSeller};

#[derive(Debug)]
struct VenueManagement<T: TicketSeller> {
    venue: T,
    manager: Option<String>,
}

impl<T: TicketSeller> VenueManagement<T> {
    fn new(venue: T) -> Self {
        Self {
            venue,
            manager: None,
        }
    }

    fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn venue_management_can_hire_manager() {
        let movie_theater = MovieTheater::new();
        let mut venue_management = VenueManagement::new(movie_theater);
        venue_management.hire_manager("Lucas");
        assert_eq!(venue_management.manager, Some(String::from("Lucas")));
    }
}
