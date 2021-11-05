extern crate exec;

use std::fs::read_dir;
use std::thread;
use std::thread::JoinHandle;
use std::path::Path;
use std::process::exit;


/*
 * Run the search for the file, and then run the file with the supplied args.
 * */
pub fn run(root: &Path, filename: String, args: Vec<&str>) {
    println!("Searching for {}", filename);

    match search(root, filename) {
        Some(name) => {
            let error = exec::Command::new("sh").arg(name).args(&args).exec();
            eprintln!("{}", error);
        }
        None => {
            eprintln!("Cannot find file specified.");
            exit(1);
        }
    }
}

/*
 * Recursively search for a matching file name, returning the name of the file if found.
 *  args:
 *      dir: the directory to search
 *      file: the file name to search for
 *  returns:
 *      Some(String) if the file is found
 *      None if the file is not found
 * */
fn search(dir: &Path, file: String) -> Option<String> {
    // Vector to hold thread references
    let mut threads: Vec<Box<JoinHandle<Option<String>>>> = Vec::new();
    for entry in read_dir(dir).unwrap() {
        let dir_entry: std::fs::DirEntry = entry.unwrap();
        let path = dir_entry.path();
        if path.is_dir() {
            // If the path is a directory, recursively search it
            let file_ref = file.clone();
            threads.push(Box::new(thread::spawn(move || search(&path, file_ref))));
        } else if let Some(name) = path.file_stem() {
            let name = name.to_string_lossy();
            if name == file {
                return Some(path.to_str().unwrap().to_owned());
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
