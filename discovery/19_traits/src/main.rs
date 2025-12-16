use traits::doctor::Appointment;
use traits::duration::Duration;
use traits::flight::{Bus, Flight};
use traits::fruits::{Apple, AppleType};
use traits::lodging::{Accommodation, AirBnB, Description, Hotel};
use traits::taxes::{Bonus, Income, Investment, QualityTime, Taxable};
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

    let income = Income { amount: 20567.34 };
    println!("${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 11500.47 };
    println!("${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("${:.2}", bonus.tax_bill());

    let time = QualityTime { time: 120 };
    println!("{}", time.get_amount());

    let lunch = Apple {
        kind: AppleType::Green,
        price: 10.0,
    };

    println!("Normal: {}", lunch);
    println!("Debug: {:?}", lunch);

    let morning_app = Appointment::new("Jeff", "09:00AM", "10:00AM");
    let new_app = morning_app.clone();
    println!("{} is seeing the patient", new_app.doctor);

    // use the copy trait to not transfer ownership
    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;
    let yet_another_hour = one_hour.clone();

    // prove that ownership is not transfered
    println!("{:?}", one_hour);
    println!("{:?}", another_hour);
    println!("{:?}", yet_another_hour);

    let br_lk1 = Flight::new("11", "BR", "LK");
    let br_lk2 = Flight::new("11", "BR", "LK");
    let us_lk = Flight::new("11", "US", "LK");

    // both syntaxes are valid here
    println!("{}", br_lk1.eq(&br_lk2));
    println!("{}", us_lk == br_lk2);

    // impl for partialeq with generic definition
    let bus_br_lk = Bus::new("36", "BR", "LK");
    println!("{}", br_lk1 == bus_br_lk);
}
