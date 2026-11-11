use std::path::Path;
use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    let path = path.as_path();
    println!("current path: {}", path.display());
    for entry in path.read_dir().expect("read_dir call failed") {
    if let Ok(entry) = entry {
        print!("{}", entry.file_name().to_str().unwrap());
    }
}
}
