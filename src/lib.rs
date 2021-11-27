extern crate exec;

use std::fs::read_dir;
use std::path::Path;
use std::process::exit;

/*
 * Run the search for the file, and then run the file with the supplied args.
 * */
pub fn run(root: &Path, filename: String, args: Vec<&str>, config: Config) {
    match search(root, &filename) {
        Some(filename) => {
            if config.cat {
                eprintln!(
                    "{}",
                    exec::Command::new("cat").arg(filename).exec()
                );
            } else {
                eprintln!(
                    "{}",
                    exec::Command::new("cat").arg(filename).args(&args).exec()
                );
            }
        }
        None => {
            eprintln!("Cannot find file specified.");
            exit(1);
        }
    }
}

/*
 * Recursive search for matching file name, running the script if found.
 * */
fn search(dir: &Path, file: &str) -> Option<String> {
    for entry in read_dir(dir).unwrap() {
        let dir_entry: std::fs::DirEntry = entry.unwrap();
        let path = dir_entry.path();
        if path.is_dir() {
            match search(&path, file) {
                Some(name) => return Some(name),
                None => continue,
            }
        }

        if let Some(name) = path.file_stem() {
            let name = name.to_string_lossy();
            if name == file {
                return Some(path.to_str().unwrap().to_owned());
            }
        }
    }
    None
}

/*
 * The values of the CLI flags
 * */
#[derive(Debug)]
pub struct Config {
    pub cat: bool,
}
