use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }

    fn get_description(&self) -> String {
        format!("Please enjoy {} apartment", self.host)
    }
}

fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) { //? Can also be declared as fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {}
    entity.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("Ibis");
    println!("{}", hotel.get_description());
    hotel.book("Dinha", 5);
    println!("{}", hotel.summarize());
    book_for_one_night(&mut hotel, "Elza");
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Lucas");
    println!("{}", airbnb.get_description());
    airbnb.book("Ale", 7);
    book_for_one_night(&mut airbnb, "Linea");
    println!("{:#?}", airbnb);
}
