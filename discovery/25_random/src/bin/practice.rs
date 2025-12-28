use rand::seq::SliceRandom;
use rand::{Rng, random, rng};

fn main() {
    let random_float: f64 = random();
    println!("{}", random_float);

    let random_int: u8 = random();
    println!("{}", random_int);

    // creates a random number generator instance that implements the Rng trait, mutable because administer internal state
    // cheaper when beign reused because random() alone creates many rng() instances
    let mut my_rng = rng();

    let my_random: u32 = my_rng.random();
    println!("{}", my_random);

    let random_values = (0..10).map(|_| my_rng.random::<i8>()).collect::<Vec<i8>>();
    println!("{:?}", random_values);

    let random_range = my_rng.random_range(0..2);
    println!("{}", random_range);

    println!("{}", my_rng.random_bool(0.5));

    let mut candies = vec![
        "Super Lemon",
        "Babaloo",
        "Kinder Bueno",
        "Super Cola",
        "Lolipop",
        "Snickers",
        "KitKat",
    ];

    // to shuffle is necessary to import rand::seq::SliceRandom;
    candies.shuffle(&mut my_rng);

    println!("{:?}", candies);
}
