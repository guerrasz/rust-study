use std::{error::Error, fmt::Display};

trait TextTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>>;
}

struct WhitespaceTransformer {
    start: bool,
    end: bool,
}

impl TextTransformer for WhitespaceTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>> {
        if text.contains("🍕") {
            return Err(Box::new(PizzaEmojiError));
        }

        let transformed = if self.start && self.end {
            text.trim()
        } else if self.start {
            text.trim_start()
        } else if self.end {
            text.trim_end()
        } else {
            text
        };

        if transformed.is_empty() {
            return Err(Box::new(EmptyStringError));
        }

        Ok(transformed.to_string())
    }
}

#[allow(unused)]
enum Case {
    Uppercase,
    Lowercase,
}

struct CaseTransformer {
    case: Case,
}

impl TextTransformer for CaseTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>> {
        match self.case {
            Case::Lowercase => Ok(text.to_lowercase()),
            Case::Uppercase => Ok(text.to_uppercase()),
        }
    }
}

#[derive(Debug)]
struct PizzaEmojiError;

impl Display for PizzaEmojiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform"
        )
    }
}

impl Error for PizzaEmojiError {}

#[derive(Debug)]
struct EmptyStringError;

impl Display for EmptyStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The string has nothing left in it. Moving on to next transform"
        )
    }
}

impl Error for EmptyStringError {}

fn apply_transformations(text: String, transformers: Vec<Box<dyn TextTransformer>>) -> String {
    let mut final_str = text;
    for t in transformers {
        match t.transform(&final_str) {
            Ok(transformed_text) => final_str = transformed_text,
            Err(error) => {
                eprintln!("Error Message: {}", error);
                continue;
            }
        };
    }
    final_str
}

//^ alternative way to produc same result with more sophisticated syntax
// fn apply_transformations(text: String, pipeline: Vec<Box<dyn TextTransformer>>) -> String {
//     pipeline.into_iter().fold(text, |accumulator, transformer| {
//         match transformer.transform(&accumulator) {
//             Ok(new_value) => new_value,
//             Err(error) => {
//                 eprintln!("Something went wrong: {error} Moving on to next transform");
//                 accumulator
//             }
//         }
//     })
// }

fn main() {
    // Input
    //let text = String::from("  homer simpson  ");
    // Output
    // Content: "HOMER SIMPSON"

    // Input
    //let text = String::from("  data  🍕  ");
    // Output
    // Error Message: Something went wrong: Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform
    // Content: "  DATA  🍕  "

    // Input
    let text = String::from("    ");
    // Output:
    // Error Message: Something went wrong: The string has nothing left in it. Moving on to next transform
    // Content: "    "

    let pipeline: Vec<Box<dyn TextTransformer>> = vec![
        Box::new(WhitespaceTransformer {
            start: true,
            end: true,
        }),
        Box::new(CaseTransformer {
            case: Case::Uppercase,
        }),
    ];

    let transformed_text = apply_transformations(text, pipeline);
    println!("Output: {transformed_text}");
}
