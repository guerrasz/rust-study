use traits::lodging::{Accommodation, AirBnB, Description, Hotel};
use traits::utils;

fn main() {
    let mut hotel = Hotel::new("Ibis");
    println!("{}", hotel.get_description());
    hotel.book("Dinha", 5);
    println!("{}", hotel.summarize());
    utils::book_for_one_night(&mut hotel, "Elza");
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Lucas");
    println!("{}", airbnb.get_description());
    airbnb.book("Ale", 7);
    utils::book_for_one_night(&mut airbnb, "Linea");
    println!("{:#?}", airbnb);

    utils::mix_and_match(&mut hotel, &mut airbnb);

    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());
}
