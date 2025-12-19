use std::collections::HashMap;

fn main() {
    let my_list = vec![1, 2, 3, 4];
    //reutrns iterator
    //let mut iterator = my_list.into_iter();
    //println!("{:?}", iterator);

    for number in &my_list {
        println!("{:?}", number);
    }

    println!("{:?}", my_list);

    let mut todos = HashMap::new();
    todos.insert("Groceries", false);
    todos.insert("Study", true);
    todos.insert("Sleep", false);

    for (todo, completion) in &todos {
        println!("Task: {}, status: {}", todo, completion)
    }
}
