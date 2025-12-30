#[derive(Debug)]
#[allow(unused)]
// can be either empty or be a node with a value and a reference (box in this case) to the next element
enum LinkedList<T> {
    Empty,
    Node { value: T, next: Box<LinkedList<T>> },
}

#[derive(Debug)]
#[allow(unused)]
// when using refs lifetimes become a concern, the outer cannot outlive the next field
enum LList<'a, T> {
    Empty,
    Node { value: T, next: &'a LList<'a, T> },
}

//^ wont compile because second would be a dangling reference
// fn create_llist<'a>() -> LList<'a, i32> {
//     let second = LList::Node {
//         value: 2,
//         next: &LList::Empty,
//     };

//     let first = LList::Node {
//         value: 1,
//         next: &second,
//     };

//     first
// }

// compiles because box is an owned value
fn create_linked_with_box() -> LinkedList<i32> {
    let second = LinkedList::Node {
        value: 2,
        next: Box::new(LinkedList::Empty),
    };

    let first = LinkedList::Node {
        value: 1,
        next: Box::new(second),
    };

    first
}

fn main() {
    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Empty),
            }),
        }),
    };

    println!("{:#?}", list);

    let end = LinkedList::Node {
        value: String::from("I'm with you"),
        next: Box::new(LinkedList::Empty),
    };

    let earfquake = LinkedList::Node {
        value: String::from("Earfquake"),
        next: Box::new(end),
    };

    let foo = LinkedList::Node {
        value: String::from("Foo"),
        next: Box::new(earfquake),
    };

    println!("{:#?}", foo);

    let second = LList::Node {
        value: 2,
        next: &LList::Empty,
    };

    let first = LList::Node {
        value: 1,
        next: &second,
    };

    println!("{:#?}", first);

    println!("{:#?}", create_linked_with_box());
}
