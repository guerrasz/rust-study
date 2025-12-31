use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
struct NumberIsNotImpressiveError;

impl Display for NumberIsNotImpressiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The number is too small")
    }
}

impl Error for NumberIsNotImpressiveError {}

// declaring dyn Error wrapped by a box makes it possible for us to return any type that implements the base Error trait
// also the error type is automatically packaged in a Box due to the definition
fn read_file_numbers(path: &str) -> Result<i32, Box<dyn Error>> {
    //^ can return IO error
    let file_contents: String = fs::read_to_string(path)?;

    //^ this is what actually is being done under the hood
    // let file_contents = match fs::read_to_string(path) {
    //     Ok(contents) => contents,
    //     Err(error) => return Err(Box::new(error)),
    // };

    //^ can return ParseIntError
    let parsed_number = file_contents.parse::<i32>()?;

    if parsed_number < 100 {
        Err(Box::new(NumberIsNotImpressiveError))
    } else {
        Ok(parsed_number)
    }
}

fn main() {
    match read_file_numbers("value.txt") {
        Ok(value) => println!("The value is {}", value),
        Err(error) => eprint!("The error is: {}", error),
    }
}
