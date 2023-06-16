#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::ignite;

pub use crate::utils::*;
pub use crate::search::*;
pub mod utils;
pub mod search;
pub mod routes;
pub mod models;

fn main() {
    ignite().mount("/", routes![routes::fitting_sequence]).launch();
    // add testcase for the filter alternative
    //let result = filter_words(data, "---L----", &['E', 'A', 'O', 'P'], &['S', 'C']);
    //println!("{:#?}", result);
}
