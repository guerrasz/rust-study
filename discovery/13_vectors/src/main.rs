use std::vec;

struct File {
    name: String,
}

struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        self.contents.push(File { name });
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }
}

fn main() {
    examples();
}

fn examples() {
    let foo: Vec<u8> = Vec::from([4, 8, 10, 12]);
    println!("{:?}", foo);
    let pizza_diameters = vec![4, 8, 10, 12];
    println!("{:?}", pizza_diameters);
}
