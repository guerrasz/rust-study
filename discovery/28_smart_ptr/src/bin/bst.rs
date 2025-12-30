use std::cmp::Ordering;

#[derive(Debug)]
enum Bst {
    Empty,
    Node {
        value: i32,
        left: Box<Bst>,
        right: Box<Bst>,
    },
}

impl Bst {
    fn new() -> Self {
        Self::Empty
    }

    fn insert(&mut self, new_value: i32) {
        match self {
            Bst::Empty => {
                *self = Bst::Node {
                    value: new_value,
                    left: Box::new(Bst::Empty),
                    right: Box::new(Bst::Empty),
                }
            }
            Bst::Node { value, left, right } => match new_value.cmp(value) {
                Ordering::Equal => (),
                Ordering::Less => left.insert(new_value),
                Ordering::Greater => right.insert(new_value),
            },
        }
    }

    fn contains(&self, target: i32) -> bool {
        match self {
            Bst::Empty => false,
            Bst::Node { value, left, right } => match target.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.contains(target),
                Ordering::Greater => right.contains(target),
            },
        }
    }
}

fn main() {
    let mut tree = Bst::new();
    tree.insert(4);
    tree.insert(1);
    tree.insert(5);
    tree.insert(2);
    tree.insert(8);
    println!("{:#?}", tree);
    println!("{}", tree.contains(3));
    println!("{}", tree.contains(20));
    println!("{}", tree.contains(5));
    println!("{}", tree.contains(4));
    println!("{}", tree.contains(1));
    println!("{}", tree.contains(8));
}
