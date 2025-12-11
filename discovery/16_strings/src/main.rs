use std::io;

fn main() {
    let mut text = String::from("I like");
    make_money(&mut text);

    let capitalized = trim_and_capitalize(&text);
    println!("{}", capitalized);

    let collection = elements("Gold!Silver!Platinum");
    println!("{:?}", collection);

    let full_name = get_identity();
    println!("Your full name is: {}", full_name);

    example();
}

fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split('!').collect()
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();

    println!("What is your first name?");
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to collect first name");

    println!("What is your first name?");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to collect last name");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn example() {
    let mut name = String::new();
    println!("What is your name?");
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Hello {}", name.trim())
        }

        Err(message) => {
            println!("There was an error {message}")
        }
    }
}
