use rocket::http::Status;
use rocket_contrib::json::Json;
use std::env::set_current_dir;

use crate::models::{InputPayload, OutputPayload};

#[post("/words", format = "json", data = "<input>")]
pub fn fitting_sequence(input: Json<InputPayload>) -> Result<Json<OutputPayload>, Status> {
    let path_to_words = "../words/";
    set_current_dir(path_to_words).expect("Failed to set current directory");

    let word = input.word.clone();
    let colors = input.colors.clone();

    let output = OutputPayload {
        words: vec![word, colors],
    };

    Ok(Json(output))
}
