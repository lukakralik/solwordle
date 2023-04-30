use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
    env::current_dir,
    io::stdin,
};

pub use crate::utils::*;
pub use crate::search::*;
pub mod utils;
pub mod search;

fn main() {
    let data = import_dataset("test", 0);
    println!("{:#?}", data);

    let res = suitable_sequences(String::from("G*T*"), vec![String::from("A"), String::from("E")], vec![String::from("B")], data);
    println!(" res {:?}", res);
}