fn main() {
    let mut my_flight = Flight::new(
        String::from("Sao Paulo"),
        String::from("Doha"),
        1500.00,
        780,
    );
    println!("{my_flight:?}");
    my_flight.itinerary();
    my_flight
        .change_destination(String::from("Colombo"))
        .increase_price();
    my_flight.itinerary();
    println!("{my_flight:?}");
    let other_flight = Flight {
        origin: String::from("Rio de Janeiro"),
        destination: String::from("Boston"),
        ..my_flight
    };
    println!("{other_flight:?}");
    examples();
}

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passangers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passangers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passangers,
        }
    }

    fn change_destination(&mut self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price *= 1.20;
        self
    }

    fn itinerary(&self) {
        println!(
            "{} -> {}, with {} passangers",
            self.origin, self.destination, self.passangers
        );
    }
}

// & example of structs and some of their properties
fn examples() {
    // create an instance of coffee
    let mocha = Coffee {
        price: 9.99,
        name: String::from("Mocha"),
        is_hot: true,
    };

    // print instance properties
    println!(
        "My {} coffee is hot? {} and costs {}",
        mocha.name, mocha.is_hot, mocha.price
    );

    // use fn to create instance
    let redbull = create_coffee(String::from("Red Bull"), 5.99, false);

    println!(
        "{} and {} and {}",
        redbull.name, redbull.price, redbull.is_hot
    );

    drink_cofee(&redbull);

    println!("{:?}", redbull);

    let mut car = Car {
        name: String::from("Lambo"),
        year: 2010,
    };

    // follow car display and update process
    car.display_car();
    car.update_year();
    car.display_car();

    // use constructor
    let toyota = Car::new(String::from("Toyota"), 2020);
    toyota.display_car();

    // use tuple struct
    let clock = Clock(10, 30);
    clock.show_time();
}

#[derive(Debug)]
struct Car {
    name: String,
    year: u32,
}

impl Car {
    // create a constructor
    fn new(name: String, year: u32) -> Self {
        Car { name, year }
    }

    // this way self does not take ownership of the instance
    fn display_car(&self) {
        println!("Car model: {}", self.name);
        println!("Car year: {}", self.year);
    }

    fn update_year(&mut self) {
        self.year = 1999
    }
}

// defining coffee struct deriving debug trait
#[derive(Debug)]
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

// tuple struct
struct Clock(u32, u32);

impl Clock {
    fn show_time(&self) {
        println!("The time is: {}:{}", self.0, self.1);
    }
}

// fn that returns a coffee
fn create_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    return Coffee {
        name,
        price,
        is_hot,
    };
}

// fn accesses the address
fn drink_cofee(coffee: &Coffee) {
    println!("Drinking my delicious {}", coffee.name);
}
