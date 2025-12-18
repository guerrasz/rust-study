// does not need lifetime annotations because returns an owned value not a ref
fn double_lenght<T>(collection: &[T]) -> usize {
    collection.len() * 2
}

// does not need because it inferred
fn last_two<T>(vector: &[T]) -> &[T] {
    if vector.len() >= 2 {
        &vector[vector.len() - 2..]
    } else {
        &vector
    }
}

// does need lifetime because it cannot be inferred due to two refs being used and one being returned
fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{}", announcement);
    &text[..5]
}

// needs annotations due to three refs being used an one returned, lifetime cannot be infered
fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{}", double_lenght(&numbers));
    println!("{:?}", last_two(&numbers));
    let first = first_five("lucasguerra", "Henlo");
    println!("{}", first);
    println!(
        "{}",
        find_string_that_has_content("programming", "dining", "gram")
    );
    example();
}

//^ Define examples after main

const COUNT: i32 = 400;

// using the ' (tick) to define a lifetime annotation
fn select_first_two<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

// struct cant outlive referene to avoid a dangling ref
#[derive(Debug)]
struct Dentist<'a> {
    name: &'a str,
}

impl<'a> Dentist<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str,
}

// this works because &str is embedded in the binary
fn static_fn() -> &'static str {
    "hello"
}

fn give_count() -> &'static i32 {
    &COUNT
}

fn example() {
    // non-lexical lifetime
    let mut data = vec![1, 2, 3];
    let slice = &mut data[..];
    println!("{:?}", slice.first());
    data.push(2);

    let cities = vec![
        String::from("SP"),
        String::from("RJ"),
        String::from("LK"),
        String::from("US"),
        String::from("QA"),
    ];

    let first = select_first_two(&cities);
    println!("{:?}", first);

    let orlando = String::from("Orlando");
    let sf = "San Franciso";
    println!("{:?}", longest(&orlando, sf));
    println!("{}", orlando);

    let dentist = Dentist::new("lucas");
    println!("{:?}", dentist.name);

    let from = String::from("BR");

    let plan = {
        let to = String::from("LK");
        let travel = TravelPlan {
            from: &from,
            to: &to,
        };

        println!("{}", travel.to);

        travel.from
    };

    println!("{}", plan);
    println!("{}", give_count());
    println!("{}", static_fn());
}
