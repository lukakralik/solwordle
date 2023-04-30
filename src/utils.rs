use std::fs;
use std::str;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

// load all the words of the given length into a vector
pub fn import_dataset(lang: &str, des_len: u8) -> Vec<String> {
    // Convert the lenght u8 into a String
    let mut len = String::new();
    len.push((des_len + b'0') as char);

    // Define the path to the data
    let abs_path = match des_len {
        0 => format!("words/{}.txt", lang),
        _ => format!("words/{}{}.txt", lang, des_len)
    };
    
    // Initialize file and reader for loading data from the file
    let file = File::open(&abs_path)
        .expect("Unable to open file");
    let reader = BufReader::new(file);

    // Load each line of the file as a new entry of the lines vector
    let mut lines = vec![];
    for line in reader.lines() {
        let line = line
            .expect("Unable to read line");
        lines.push(line);
    }

    to_upper(lines)
}

// check if string contains the given symbol
pub fn contains(symbol: char, sequence: &str) -> bool {
    sequence
        .chars()
        .any(|c| c == symbol)
}

// turn all the elements of the vector uppercase
fn to_upper(list: Vec<String>) -> Vec<String> {
    let mut vec = list;
    for s in vec.iter_mut() {
        *s = s.to_uppercase();
    }
    vec
}