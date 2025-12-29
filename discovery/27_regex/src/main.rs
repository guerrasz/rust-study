use regex::Regex;

fn main() {
    let re = Regex::new(r"ue").unwrap();
    let text = "my movie queue";

    match re.find(text) {
        Some(data) => {
            println!("{:?}", data)
        }
        None => println!("No match found"),
    }
}
