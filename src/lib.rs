extern crate exec;

use std::fs::{read_dir};
use std::path::Path;
use std::process::exit;

/*
 * Run the search for the file, and then run the file with the supplied args.
 * */
pub fn run(root: &Path, filename: String, args: Vec<&str>) {
    println!("Searching for {}", filename);

    if let Some(filename) = search(root, filename) {
        let error = exec::Command::new("sh")
            .arg(filename)
            .args(&args)
            .exec();
        eprintln!("{}", error);
    } else {
        eprintln!("Cannot find file specified.");
        exit(1);
    }
}

/*
 * Recursive search for matching file name, running the script if found.
 * */
fn search(dir: &Path, file: String) -> Option<String>{
    for entry in read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            return search(&path, file);
        }

        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy();
            if name == file {
                return Some(path.to_str().unwrap().to_owned());
            }
        }
    }
    None
}
