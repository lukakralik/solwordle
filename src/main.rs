use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
    env::current_dir,
    io::stdin,
};

pub use crate::utils::*;
pub mod utils;
pub mod levenshtein;
pub mod tests;

fn main() {
    let data = utils::import_dataset("german", 4);
    println!("{:#?}", data);
}