#[macro_use]
extern crate clap;

use std::env::current_dir;
use std::path::Path;

use clap::App;
use sd::{run, Config};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut dir: &Path = &current_dir().unwrap();
    let mut args: Vec<&str> = vec![];
    let script = matches.value_of("FILE").unwrap().to_string();

    if matches.is_present("ARGS") {
        args = matches.values_of("ARGS").unwrap().collect();
    }

    if matches.is_present("directory") {
        dir = Path::new(matches.value_of("directory").unwrap());
    }

    let config = Config {
        cat: matches.is_present("cat"),
    };

    run(dir, script, args, config);
}
