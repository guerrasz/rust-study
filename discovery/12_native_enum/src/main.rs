use std::num::ParseIntError;

#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice {
            None
        } else if self.reservations < 12 {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Strip Steak"),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice {
            Result::Err(String::from("Sorry we have a problem"))
        } else if address.is_empty() {
            Result::Err(String::from("No address specified"))
        } else {
            Result::Ok(Food {
                name: String::from("Burger"),
            })
        }
    }
}

fn main() {
    let restaurant = Restaurant {
        has_mice: true,
        reservations: 11,
    };

    println!("{:?}", restaurant.chef_special());
    println!("{:?}", restaurant.deliver_burger(""));

    let restaurant = Restaurant {
        has_mice: false,
        reservations: 19,
    };

    let special = restaurant.chef_special().unwrap_or(Food {
        name: String::from("No Food"),
    });

    println!("{:?}", special.name);

    examples();
}

fn examples() {
    let foo = [
        String::from("bar"),
        String::from("buzz"),
        String::from("zit"),
    ];

    let zit = foo.get(2);
    extract(zit);

    let invalid = foo.get(777);
    extract(invalid);

    println!("{:?}", invalid.unwrap_or(&String::from("Nothing at all")));

    let ok: Result<i8, &str> = Result::Ok(8);
    println!("{:?}", ok);
    let disaster: Result<i8, &str> = Result::Err("FooBar");
    println!("{:?}", disaster);

    let numcorr = "50";
    let nonnum = "alabama";

    println!("{:?}", parse_txt(numcorr));
    println!("{:?}", parse_txt(nonnum));
}

fn extract(variable: Option<&String>) {
    match variable {
        Option::Some(foo) => {
            println!("{foo}")
        }

        Option::None => {
            println!("None values")
        }
    }
}

fn parse_txt(text: &str) -> Result<i8, ParseIntError> {
    text.parse::<i8>()
}
