use std::process::exit;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let password = procedure();
        if password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Location {
    name: String,
    population: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        for loc in self.locations {
            action(loc)
        }
    }
}

fn main() {
    let mult = 5;

    //& does not work due to scope
    // fn multiply(value: i32) -> i32 {
    //     value * mult
    // }

    // able to access mult from line 2
    let multiply_by = |value: i32| -> i32 { value * mult };

    println!("{:?}", multiply_by(3));

    let numbers = vec![1, 2, 2, 3];

    // Fn() because function uses an immutable reference
    let print_numbers = || println!("{:?}", numbers);

    print_numbers();

    let mut primes = vec![1, 3, 7];

    // FnMut as uses a mutable reference to primes as taking ownership is not needed but mutates value
    // Has to be declared as mut due to the state of primes changing agter each invocation so it has to update itself therefore mut
    let mut add_number = || primes.push(3);
    //? error because primes is borrows as mut in the line above
    // println!("{:?}", primes);
    add_number();
    add_number();
    println!("{:?}", primes);

    //^ Closures with ownership
    let number = 5;
    // number lives on the stack and implements copy so it will not take ownership
    let capture_number = || number;
    println!("{} {}", capture_number(), number);

    let name = String::from("Lucas");

    // FnOnce since it will take ownership of the name value so it can only once
    let capture_string = || name;
    //? error because name is not the owner
    //? println!("{}", name);
    let foo = capture_string();
    println!("{}", foo);

    let first_name = String::from("Loo");

    let print_name = move || println!("{}", first_name);
    //^ Error because the closure moves ownership but it still is Fn()
    //println!("{}", first_name);
    print_name();

    let option = Some("Carbonara");
    // closure being passed as a parameter to provid fallback behavior that has access to the current scope
    let food = option.unwrap_or_else(|| "Pizza");
    println!("{}", food);

    let vault = Vault {
        password: String::from("pass"),
        treasure: String::from("Gold"),
    };

    let mut user_input = String::new();
    println!("Please provide the password:");

    if let Err(error) = std::io::stdin().read_line(&mut user_input) {
        eprint!("Error obtaining input. {error}");
        exit(1);
    }

    user_input = user_input.trim().to_string();

    let hack = || user_input;

    match vault.unlock(hack) {
        Some(treasure) => println!("Treasure is: {}", treasure),
        None => println!("Wrong password, self destructing!"),
    }

    let mut bar = String::from("Guerra");
    let get_lower = |char: char| {
        if char.is_uppercase() { false } else { true }
    };
    bar.retain(get_lower);
    println!("{}", bar);

    // create map
    let map = Map {
        locations: &vec![
            Location {
                name: String::from("US"),
                population: 10000,
            },
            Location {
                name: String::from("LK"),
                population: 15000,
            },
            Location {
                name: String::from("BR"),
                population: 27000,
            },
        ],
    };

    let mut total_population = 0;

    map.explore(|loc: &Location| {
        println!("I love: {}", loc.name);
        total_population += loc.population;
    });
    println!("Total population visted: {}", total_population)
}
