pub const PERSONAL_TRAINER: &str = "Will Weight";

pub fn ask_about_program() {
    println!("The wightligting trainer is {}", PERSONAL_TRAINER);
}

#[derive(Debug)]
pub struct Exercise {
    pub name: String,
    pub reps: u32,
}

impl Exercise {
    pub fn new(name: String, reps: u32) -> Self {
        Exercise { name, reps }
    }
}
