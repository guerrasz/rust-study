use std::ops::{Deref, DerefMut};

struct CustomBox<T> {
    data: T,
}

impl<T> CustomBox<T> {
    fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for CustomBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        println!("cleaning related values")
    }
}

fn main() {
    let mut custom_box = CustomBox::new(44);
    println!("{}", *custom_box);
    *custom_box = 28;
    println!("{}", *custom_box)
}
