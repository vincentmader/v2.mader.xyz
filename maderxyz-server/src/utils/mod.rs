
// ===================================================

use std::collections::HashMap;

extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub page_id: String,  // pub category: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pages: HashMap<String, Page>,
}

pub fn load_config() -> Config {
    let file_content = load_file("../config.json");
    let config = serde_json::from_str(&file_content).unwrap();
    config
}

// ===================================================

use std::fs;

pub fn load_file(path_to_file: &str) -> String {
    fs::read_to_string(path_to_file)
        .expect("Something went wrong reading the file")
}

