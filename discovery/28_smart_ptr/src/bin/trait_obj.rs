use std::vec;

trait Wearable {
    fn wear(&self) -> String;
}

#[derive(Debug)]
struct Pants {
    fabric: String,
}

impl Wearable for Pants {
    fn wear(&self) -> String {
        format!("{} pants", self.fabric)
    }
}

#[derive(Debug)]
struct Tie {
    color: String,
}

impl Wearable for Tie {
    fn wear(&self) -> String {
        format!("{} tie", self.color)
    }
}

fn main() {
    let pants = Pants {
        fabric: String::from("Jeans"),
    };

    let tie = Tie {
        color: String::from("Pink"),
    };

    let outfit: Vec<Box<dyn Wearable>> = vec![Box::new(pants), Box::new(tie)];

    for piece in outfit {
        println!("Putting on the {}", piece.wear())
    }
}
