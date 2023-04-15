use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
    env::current_dir,
    io::stdin,
};

pub mod utils;
pub mod levenshtein;
pub use crate::utils::*;

fn main() {
    let data = utils::import_dataset("../words/test.txt");
    println!("{:#?}", data);
}
