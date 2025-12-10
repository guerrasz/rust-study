// import modules
// mod inventory;
// mod orders;

// use stdlib modules
#[allow(unused_imports)]
use std::{fmt, io::*};

// use external modules
use fake::{Fake, Faker};

// use internal modules
use project_structure::{INV_MANAGER, Item, ORD_MANAGER, ProductCategory};

// use inventory::MANAGER as INV_MANAGER;
// use inventory::products::{Item, ProductCategory};
// use orders::MANAGER as ORD_MANAGER;

fn main() {
    let hammer = Item {
        name: String::from("Hammer"),
        category: ProductCategory::Hammer,
    };
    println!("{:?}", hammer);
    println!("{:?} {:?}", hammer.name, hammer.category);

    let ladder = Item::new(String::from("Ladder"), ProductCategory::Ladder);
    println!("{:?}", ladder);

    println!("{}", ORD_MANAGER);
    println!("{}", INV_MANAGER);

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}
