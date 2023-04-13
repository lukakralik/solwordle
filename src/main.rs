use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
    env::current_dir,
    io::stdin,
};
use rand::Rng;

fn main() {
    let data = import_dataset("../words/english.txt");
}

// move to utils
fn import_dataset(path: &str) -> Vec<String> {
    let data: Vec<String> = fs::read_to_string(path)
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let words: Vec<String> = data
        .iter()
        .map(String::from)
        .filter(|x| x.len() == 6)
        .collect();
    words
}

// move to utils
fn contains(symbol: char, sequence: &str) -> bool {
    for c in sequence.chars() {
        if c == symbol {
            return true
        }
        else {
            continue
        }
    }
    false
}
