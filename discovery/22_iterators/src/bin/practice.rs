use std::collections::HashMap;

fn count_chars(text: &str) -> HashMap<char, u8> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        }
    }

    counts
}

fn main() {
    let my_list = vec![1, 2, 3, 4];
    //reutrns iterator
    //let mut iterator = my_list.into_iter();
    //println!("{:?}", iterator);

    for number in &my_list {
        println!("{:?}", number);
    }

    println!("{:?}", my_list);

    let mut todos = HashMap::new();
    todos.insert("Groceries", false);
    todos.insert("Study", true);
    todos.insert("Sleep", false);

    for (todo, completion) in &mut todos {
        println!("Task: {}, status: {}", todo, completion)
    }

    let seafood = "Shrimp🍤";

    for byte in seafood.bytes() {
        println!("{}", byte)
    }

    for char in seafood.chars() {
        println!("{}", char)
    }

    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());

    println!(
        "{:?}",
        count_chars("Lucas is giving his best learning rust")
    );

    let numbers = vec![1, 2, 4, 8, 9, 13, 21];
    // .iter() turns numbers into iterator
    // .map() makes the actions inside the closure for each item
    // .collect() collects in the resulting iterator again
    let squares: Vec<u32> = numbers.iter().map(|number: &u32| number.pow(2)).collect();

    println!("{:?}", squares);

    let names = ["Lucas", "Rafel", "Allan", "Felipe"];

    let name_lenghts: Vec<usize> = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect();

    println!("{:?}", name_lenghts);

    let evens: Vec<u32> = numbers
        .into_iter()
        .filter(|number| number % 2 == 0)
        .collect();

    println!("{:?}", evens);
}
