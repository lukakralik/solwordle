use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::env;

// load all the words of the given length into a vector
pub fn import_dataset(lang: &str, des_len: u8) -> Vec<String> {
    // Convert the lenght u8 into a String
    let len = (des_len + b'0') as char;

    // Define the path to the data
    let mut abs_path = PathBuf::from(env::current_dir().unwrap());
    abs_path.push("words");

    let filename = match des_len {
        0 => format!("{}.txt", lang),
        _ => format!("{}{}.txt", lang, len)
    };
    abs_path.push(filename);

    let file = File::open(&abs_path)
        .expect("Unable to open file");
    let reader = BufReader::new(file);

    // Load each line of the file as a new entry in the lines vector
    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Unable to read line"))
        .collect();

    to_upper(lines)
}

// check if string contains the given symbol
pub fn contains(symbol: char, sequence: &str) -> bool {
    sequence
        .chars()
        .any(|c| c == symbol)
}

// turn all the elements of the vector uppercase
pub fn to_upper(list: Vec<String>) -> Vec<String> {
    let mut vec = list;
    for s in vec.iter_mut() {
        *s = s.to_uppercase();
    }
    vec
}