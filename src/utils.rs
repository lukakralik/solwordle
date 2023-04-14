use std::fs;

pub fn import_dataset(path: &str) -> Vec<String> {
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
