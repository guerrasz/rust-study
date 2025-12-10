use std::vec;

#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
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

    fn get_file(&mut self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new(String::from("Games"));
    folder.create_file(String::from("RD2"));
    folder.create_file(String::from("Valorant"));
    println!("{:?}", folder);
    println!("{:?}", folder.name);
    folder.delete_file(1);
    println!("{:?}", folder);
    let file = folder.get_file(0);
    match file {
        Some(content) => println!("{:?}", content.name),
        None => println!("No file was found"),
    }
    examples();
}

fn examples() {
    let foo: Vec<u8> = Vec::from([4, 8, 10, 12]);
    println!("{:?}", foo);
    let pizza_diameters = vec![4, 8, 10, 12];
    println!("{:?}", pizza_diameters);
}
