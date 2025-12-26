#![allow(unused)]
use crate::attractions::TicketSeller;

#[derive(Debug)]
pub struct VenueManagement<T: TicketSeller> {
    pub venue: T,
    manager: Option<String>,
}

impl<T: TicketSeller> VenueManagement<T> {
    pub fn new(venue: T) -> Self {
        Self {
            venue,
            manager: None,
        }
    }

    fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    pub fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // create a dummy venue to avoid other code compelxity of functionality/latency during testing
    struct DummyVenue {}

    impl TicketSeller for DummyVenue {
        fn sell_ticket(&mut self) {}
    }

    #[test]
    fn venue_management_can_hire_manager() {
        let mut venue_management = VenueManagement::new(DummyVenue {});
        venue_management.hire_manager("Lucas");
        assert_eq!(venue_management.manager, Some(String::from("Lucas")));
    }
}
