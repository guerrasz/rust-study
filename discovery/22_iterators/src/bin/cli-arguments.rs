use std::env;

fn main() {
    //^ args struct implements the iterator trait
    let arguments = env::args();
    for arg in arguments {
        println!("{}", arg)
    }
}
