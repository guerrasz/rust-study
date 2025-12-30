fn main() {
    let mut sushi = String::from("Sushi");

    // raw pointer immutable
    let sushi_raw_ptr_1 = &raw const sushi;
    let sushi_raw_ptr_2 = &raw const sushi;

    // compiler lets 2 mutable ptr coexist
    let sushi_ptr_mut_1 = &raw mut sushi;
    let sushi_ptr_mut_2 = &raw mut sushi;

    // drops the value from the heap
    drop(sushi);

    //^ to dereference the pointer, its necessary to add the unsafe block
    unsafe {
        println!("{}", *sushi_raw_ptr_1);
        println!("{}", *sushi_raw_ptr_2);
        println!("{}", *sushi_ptr_mut_1);
        println!("{}", *sushi_ptr_mut_2);
    }
}