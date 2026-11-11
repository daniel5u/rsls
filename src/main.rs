// use std::path::Path;
// use std::ffi::OsString;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn mode_string(meta: &std::fs::Metadata) -> String {
    let mode = meta.permissions().mode();
    let mut ans = String::with_capacity(10);

    if meta.is_dir() {
        ans.push('d');
    } else if meta.is_symlink() {
        ans.push('l');
    } else {
        ans.push('-');
    }

    for shift in [6, 3, 0] {
        let bits = (mode >> shift) & 0o7;
        ans.push(if bits & 0o4 != 0 { 'r' } else { '-' });
        ans.push(if bits & 0o2 != 0 { 'w' } else { '-' });
        ans.push(if bits & 0o1 != 0 { 'x' } else { '-' });
    }

    ans
}

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

    let mut file_names: Vec<String> = vec![];
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
        file_names = file_names
            .into_iter()
            .filter(|n| !n.starts_with('.'))
            .collect();
    }

    let mut final_outputs: Vec<String> = vec![];

    // Metadata { file_type: FileType { is_file: true, is_dir: false, is_symlink: false, .. }, permissions: Permissions(FilePermissions { mode: 0o100644 (-rw-r--r--) }), len: 148, modified: SystemTime { tv_sec: 1794309714, tv_nsec: 740066618 }, accessed: SystemTime { tv_sec: 1794362050, tv_nsec: 479011910 }, created: SystemTime { tv_sec: 1794309714, tv_nsec: 739943160 }, .. }

    if arguments.iter().any(|a| a.eq("-l")) {}

    for f in file_names.iter() {
        if arguments.iter().any(|a| a.eq("-l")) {
            let metadata: fs::Metadata = fs::metadata(f).unwrap();
            final_outputs.push(mode_string(&metadata));
            final_outputs.push(String::from("  "));
        }
        final_outputs.push(String::from(f));
        if arguments.iter().any(|a| a.eq("-l")) {
            final_outputs.push(String::from("\n"));
        } else {
            final_outputs.push(String::from("  "));
        }
    }

    final_outputs.iter().for_each(|fo| print!("{fo}"));
}
