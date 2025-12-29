use regex::Regex;

/*
    anything capitalized inverses the condition
    \s -> whitespace
    \d -> digits
    \w -> alphanumeric
    \b -> boundaries between words (usually use in combination with something like \bc finds a c after a boundary)
    . -> matches all characters, also used with other things (.i finds a combination of anything + i) (\. finds a dot)
    [] -> defines a set of characters to be matched [vri] finds any letter inside the [], you can define a range with [a-l]
*/

fn main() {
    let re = Regex::new(r"ue").unwrap();
    let text = "my movie queue";

    // find first match in haystack
    match re.find(text) {
        Some(data) => {
            println!("Match is: {:?}", data)
        }
        None => println!("No match found"),
    }

    // find all matches in haystack
    for data in re.find_iter(text) {
        println!("{:?}", data)
    }

    // when inverted to \D we would get all characters that are not digits
    let re = Regex::new(r"\d").unwrap();
    let text = "My ZIP code is 90210. I am very rich.";

    // find all matches in haystack
    for data in re.find_iter(text) {
        println!("{:?}", data)
    }

    // alphanumerics, when capitalized inverts the condition
    let re = Regex::new(r"\w").unwrap();
    let text = "My ZIP code is 90210. I am very rich.";

    // find all matches in haystack
    for data in re.find_iter(text) {
        println!("{:?}", data)
    }

    // define captures to isolate chunks of the haystack
    let re = Regex::new(r"(?<street_number>\d+)(.+)(?<state>\w{2})").unwrap();
    let text = "123 Elm Street, Palm Springs, CA";

    let captures = re.captures(text).unwrap();
    println!("{}", &captures[0]);
    println!("{}", &captures[1]);
    println!("{}", &captures["street_number"]);
    println!("{}", &captures[2]);
    println!("{}", &captures["state"]);
    println!("{}", &captures[3]);

    let re = Regex::new(r"(?<count>\d+)").unwrap();
    let text = "I have 2 apples and 10 bananas";

    let result = re.replace_all(text, "$count delicious");
    println!("{}", result);
}
