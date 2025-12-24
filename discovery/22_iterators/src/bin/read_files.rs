use std::fs;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: read_files <path>",);
        eprintln!("Error: No path specified as CLI argument");
        std::process::exit(1);
    });

    match read_path_contents(&path) {
        Ok(_) => std::process::exit(0),
        Err(error) => eprintln!("Error reading file, {error}"),
    }
}

fn read_path_contents(path: &str) -> std::io::Result<()> {
    for dir_entry in fs::read_dir(path)? {
        let entry = dir_entry?;
        if entry.file_type()?.is_file() {
            println!("Filename: {:?}\n --------------", entry.file_name());
            println!("{}", fs::read_to_string(entry.path())?);
        }
    }

    Ok(())
}
