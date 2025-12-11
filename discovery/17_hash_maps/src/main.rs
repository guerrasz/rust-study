use std::collections::HashMap;

fn main() {
    let values = [
        (
            "Ketchup",
            Vec::from(["French Fries", "Burgers", "Hot Dogs"]),
        ),
        ("Mayonese", Vec::from(["Sandwiches", "Burgers", "Coleslaw"])),
    ];
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from(values);

    sauces_to_meals.insert("Mustard", Vec::from(["Hot dog", "Burgers", "Pretzels"]));

    remove_key("Mayonese", &mut sauces_to_meals);

    match sauces_to_meals.get("Mustard") {
        Some(meals) => {
            println!("{:?}", meals)
        }
        None => {
            println!("No value found")
        }
    }

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(Vec::from(["Sushi", "Dumplings"]));

    println!("{:?}", sauces_to_meals);

    examples();
}

fn remove_key(key: &str, set: &mut HashMap<&str, Vec<&str>>) {
    if let Some(meals) = set.remove(key) {
        println!("{:?}", meals)
    } else {
        println!("{} key not found", key)
    }
}

fn examples() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("Carbonara"), 29.99);
    menu.insert(String::from("Tuna"), 18.99);
    menu.insert(String::from("Steak"), 15.99);

    println!("{:?}", menu);
    println!("{:?}", menu.get("Tuna"));

    menu.entry(String::from("Sushi")).or_insert(5.99);

    println!("{:?}", menu)
}
