#![allow(unused)]

pub trait TicketSeller {
    fn sell_ticket(&mut self);
}

#[derive(Debug, PartialEq, Eq)]
struct Museum {
    paitings: Vec<String>,
    revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    fn new() -> Self {
        Self {
            paitings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        if self.paitings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting")
        }
        self.paitings.push(painting.to_string());
    }

    fn has_impressive_collection(&self) -> bool {
        self.paitings.len() > 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MovieTheater {
    movies: Vec<String>,
    sales: u32,
}

impl MovieTheater {
    pub fn new() -> Self {
        Self {
            movies: vec![],
            sales: 0,
        }
    }

    fn add_movie(&mut self, movie: &str) {
        self.movies.push(movie.to_string());
    }
}

impl TicketSeller for MovieTheater {
    fn sell_ticket(&mut self) {
        self.sales += 15;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    // unit test using the Result enum instead of the common macros
    fn museum_sells_ticket_to_increase_revenue() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.sell_ticket();
        if museum.revenue == 25 {
            Ok(())
        } else {
            Err(String::from(
                "The revenue from ticket sell did not meet expectations.",
            ))
        }
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_collection() {
        let mut museum = Museum::new();
        museum.buy_painting("Foo");
        museum.buy_painting("Bar");
        museum.buy_painting("Buz");
        assert!(
            museum.has_impressive_collection(),
            "The museum did not have an impressive collection despite having 3 paintings"
        )
    }

    #[test]
    #[should_panic(expected = "storage")]
    fn museum_prohibits_adding_painting_when_is_full() {
        let mut museum = Museum::new();
        museum.buy_painting("Foo");
        museum.buy_painting("Bar");
        museum.buy_painting("Buz");
        museum.buy_painting("Buz");
    }

    #[test]
    fn museum_revenue_is_not_zero_after_ticket() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_ne!(museum.revenue, 0);
    }

    #[test]
    fn new_museums_are_equal() {
        let museum_a = Museum::new();
        let museum_b = Museum::new();
        assert_eq!(museum_a, museum_b);
    }
}
