use std::ops::Add;

use crate::lodging::{Accommodation, Description};

pub fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    //? Can also be declared as fn book_for_one_night(entity: &mut (impl Accommodation + Description), guest: &str) {}
    entity.book(guest, 1);
}

//^ Third way to declare generics in parameters of functions
pub fn mix_and_match<T, U>(first: &T, second: &mut U)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.get_description();
    second.book("Digas", 2);
}

// defining a add function that uses generics and defines the output associated value
pub fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
