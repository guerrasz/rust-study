#[derive(Debug)]
#[allow(unused)]
enum Filesystem {
    File {
        name: String,
    },
    Folder {
        name: String,
        // vector is a smart pointer for it does not break compilation with indirection
        content: Vec<Filesystem>,
    },
}

fn main() {
    // smart pointer, wraps a raw pointer
    let my_box = Box::new(100);
    println!("{:?}", my_box);

    let rust_file = Filesystem::File {
        name: String::from("foo.rs"),
    };

    let go_file = Filesystem::File {
        name: String::from("foo.go"),
    };

    let folder = Filesystem::Folder {
        name: String::from("Code"),
        // both variables loose ownership and it's moved to content
        content: vec![rust_file, go_file],
    };

    println!("{:#?}", folder);
}
