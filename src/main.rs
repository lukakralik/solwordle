use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
    env::current_dir,
    io::stdin,
};
use crate::search::*;

pub use crate::utils::*;
pub use crate::search::*;
pub mod utils;
pub mod search;

fn main() {
    let data = import_dataset("english", 8);
    let data2 = data.clone();
    //println!("{:#?}", data);

    //let res = suitable_sequences(String::from("G*T*"), vec![String::from("A"), String::from("E")], vec![String::from("B")], data);
    //println!(" res {:?}", res);
    
    // add testcase for the filter alternative
    let result = filter_words(data2, "---L----", &['E', 'A', 'O', 'P'], &['S', 'C']);
    println!("{:#?}", result);
}
