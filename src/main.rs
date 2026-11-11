// use std::path::Path;
// use std::ffi::OsString;
use std::env;

fn main() {

    // Prints each argument on a separate line
    let mut arguments: Vec<String> = vec![];
    for argument in env::args() {
        arguments.push(argument);
    }
    println!("All the arguments:");
    arguments.iter().for_each(|a| print!("`{}`  ", a));
    println!("");

    let path = env::current_dir().unwrap();
    let path = path.as_path();
    println!("current path: {}", path.display());

    let mut file_names: Vec<String>= vec![];
    file_names.push(String::from("."));
    file_names.push(String::from(".."));
    for entry in path.read_dir().expect("read_dir call failed") {
    if let Ok(entry) = entry {
        let file_name = entry.file_name();
        file_names.push(file_name.into_string().unwrap());
        }
    }

    // Sort the files. Starting with `.` will be the least (same as `ls` logic)
    file_names.sort();

    // ignore the 'dotfiles'
    if !arguments.iter().any(|a| a.eq("-a")) {
        file_names = file_names.into_iter().filter(|n| !n.starts_with('.')).collect();
    }

    let mut final_outputs: Vec<String> = vec![];

    file_names.into_iter().for_each(|n| final_outputs.push(n));
}
