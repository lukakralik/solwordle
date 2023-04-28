use std::fs;
use std::str;
use std::env;

pub fn import_dataset(path: &str, des_len: u8) -> Vec<String> {
    let mut abs_path = env::current_dir()
        .expect("Failed to get current directory");
    abs_path.push(path);

    let data: Vec<String> = fs::read_to_string(abs_path)
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let words: Vec<String> = data
        .iter()
        .map(String::from)
        .filter(|x| x.len() == des_len as usize)
        .collect();
    to_upper(words)
}

pub fn contains(symbol: char, sequence: &str) -> bool {
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

fn to_upper(list: Vec<String>) -> Vec<String> {
    let mut vec = list;
    for s in vec.iter_mut() {
        *s = s.to_uppercase();
    }
    vec
}
