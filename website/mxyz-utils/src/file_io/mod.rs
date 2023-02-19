
use std::fs;

pub fn load_file(path_to_file: &str) -> String {
    fs::read_to_string(path_to_file)
        .expect("ERROR: Something went wrong while reading the file")
}

