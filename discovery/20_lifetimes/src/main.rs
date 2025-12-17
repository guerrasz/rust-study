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

fn main() {
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
}
