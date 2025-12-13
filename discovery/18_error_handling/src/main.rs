use std::fs;
use std::io::{self, stdin};
use std::process;

fn main() {
    match write_to_file() {
        Ok(filename) => println!("Successfully wrote to file {}", filename),
        Err(error) => println!("There was an error: {}", error),
    }
    example();
}

fn write_to_file() -> Result<String, io::Error> {
    let mut filename = String::new();
    let mut content = String::new();

    println!("What file would you like to write to?");
    stdin().read_line(&mut filename)?;

    println!("What would you like to write to the file?");
    stdin().read_line(&mut content)?;

    fs::write(&filename, &content)?;

    Ok(filename)
}

fn example() {
    match read_file() {
        Ok(content) => println!("{}", content),
        Err(error) => {
            eprint!("There was an error {:?}", error);
            process::exit(1)
        }
    }

    let mut animals = vec!["Foo", "Bar", "Buzz"];
    println!("{:?}", lenght_of_last_element(&mut animals));
}

// Using ? with Optio enum
fn lenght_of_last_element(list: &mut Vec<&str>) -> Option<usize> {
    Some(list.pop()?.len())
}

// Using ? with Result enum
fn read_file() -> Result<String, io::Error> {
    println!("Which file do you wish to read?");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(&input.trim())
}
