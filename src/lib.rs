extern crate exec;

use std::fs::read_dir;
use std::thread;
use std::thread::JoinHandle;
use std::path::Path;
use std::process::exit;

/*
 * Run the search for the file, and then run the file with the supplied args.
 * */
pub fn run(root: &Path, filename: &'static str, args: Vec<&str>) {
    println!("Searching for {}", filename);

    match search(root, filename) {
        Some(filename) => {
            let error = exec::Command::new("sh").arg(filename).args(&args).exec();
            eprintln!("{}", error);
        }
        None => {
            eprintln!("Cannot find file specified.");
            exit(1);
        }
    }
}

/* Recursively search for a matching file name, returning the name of the file if found.
    args:
        dir: the directory to search
        file: the file name to search for
    returns:
        Some(String) if the file is found
        None if the file is not found
*/
fn search(dir: &Path, file: &str) -> Option<String> {
    // Vector to hold thread references
    let mut threads: Vec<Box<JoinHandle<Option<String>>>> = Vec::new();
    for entry in read_dir(dir).unwrap() {
        let dir_entry: std::fs::DirEntry = entry.unwrap();
        let path = dir_entry.path();
        if path.is_dir() {
            // If the path is a directory, recursively search it
            threads.push(Box::new(thread::spawn(move || search(&path, file))));
        } else {
            // Check if the file name matches
            if path.file_name().unwrap() == file {
                return Some(path.to_str().unwrap().to_string());
            }
        }
    }

    // Check if any of the threads returned a value
    for thread in threads {
        match thread.join().unwrap() {
            Some(filename) => return Some(filename),
            None => continue
        }
    }

    None
}
