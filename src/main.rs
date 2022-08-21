#![allow(unused_imports, unused_variables, dead_code, unused_assignments)]
use colorful::{Color, Colorful};
use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
    env::current_dir,
    io::stdin,
};
use rand::Rng;

fn main() {
    let data = import_dataset("/home/kralikl/Programming/wordle/words");
    let vocabulary = clean_dataset(data);
    game(vocabulary);
}

fn import_dataset(path: &str) -> Vec<String> {
    let data = fs::read_to_string(path)
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
   data
}

fn clean_dataset(data: Vec<String>) -> Vec<String> {
    let words: Vec<String> = data
        .iter()
        .map(String::from)
        .filter(|x| x.len() == 6)
        .collect();
    words
}

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
//does the word really exist fn
//
//store words guessed by user somewhere

fn game(vocabulary: Vec<String>) {
    let mut rng = rand::thread_rng();
    let random_pt = rng.gen_range(0..vocabulary.len());
    let secret = vocabulary[random_pt].to_uppercase();
    //add some kind of graphics maybe
    //
    //propably split this into more modules
    println!("Enter your guess!");
    for i in 1..6 {
        let mut guess = String::new();
        stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");
         for (j, c) in guess.chars().enumerate() {
            let color: Color;
            if guess.as_bytes()[j] == secret.as_bytes()[j] {
                color = Color::Green;
            }
            else if contains(c, &secret) == true {
                color = Color::Red;
            }
            else {
                color = Color::LightGray
            }
            print!("{}", c);
            //solve colours for succesfull guess
        }
        if guess == secret {
            println!("Good job! Secret word found in round {}/6.", i);
            break;
        }
    }
}
