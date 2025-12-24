use std::fs;

fn main() -> std::io::Result<()> {
    for dir_entry in fs::read_dir("./")? {
        let entry = dir_entry?;
        if entry.file_type()?.is_file() {
            println!("Filename: {:?}\n --------------", entry.file_name());
            println!("{}", fs::read_to_string(entry.path())?);
        }
    }
    Ok(())
}
