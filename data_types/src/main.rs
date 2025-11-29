fn main() {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';
    let string: &str = "Hello, Rust!";
    let tuple: (i32, f64, char) = (integer, float, character);
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    dbg!(integer, float, boolean, character, string, tuple, array);
}
