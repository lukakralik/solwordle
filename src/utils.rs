use std::fs;
use std::str;

pub fn import_dataset(path: &str) -> Vec<String> {
    let data: Vec<String> = fs::read_to_string(path)
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let words: Vec<String> = data
        .iter()
        .map(String::from)
        //.filter(|x| x.len() == 6) filter out words by length
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


#[cfg(test)]
mod tests {
    #[test]
    fn to_upper_test() {
        let input = vec![String::from("hello"), String::from("world")];
        let output = vec![String::from("HELLO"), String::from("WORLD")];
        assert_eq!(crate::utils::to_upper(input), output);
    }

    #[test]
    fn contains_test() {
        let input = "world";
        assert_ne!(crate::utils::contains('x', input), true);
    }
}


fn to_upper(list: Vec<String>) -> Vec<String> {
    let mut vec = list;
    for s in vec.iter_mut() {
        *s = s.to_uppercase();
    }
    vec
}
