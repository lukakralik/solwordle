use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::utils::*;
use crate::search::*;
use crate::models::{InputPayload, OutputPayload};

#[post("/words", format = "json", data = "<input>")]
pub fn fitting_sequence(input: Json<InputPayload>) -> Result<Json<OutputPayload>, (Status, Vec<String>)> {
    let length = input.length.clone();
    let data = import_dataset(&input.lang.clone(), length);

    let mut include: Vec<char> = vec![];
    let mut exclude: Vec<char> = vec![];
    let dashes: String = create_hyphen_string(length);
    let pos: &str = &dashes;

    let word = input.word.clone();
    let colors = input.colors.clone();

    if word.len() != colors.len() {
        return Err((Status::BadRequest, vec![String::from("Word and colors length do not match!")]));
    }          

    for i in 0..=length {
        println!("{}", colors);
        let character = colors.chars().nth(i.into()).unwrap();
        // get exact positions for green characters
        if character == 'G' {
            set_char_at_index(pos, i.into(), word.chars().nth(i.into()).unwrap());
        }
        else if character == 'Y' {
            include.push(word.chars().nth(i.into()).unwrap());
        }
        else if character == 'W' {
            exclude.push(word.chars().nth(i.into()).unwrap());
        }
        else {
            return Err((Status::BadRequest, vec![String::from("Invalid color!")]));
        }
    }

    let result = filter_words(data, pos, include.as_slice(), exclude.as_slice());
    println!("{:#?}", result);

    let output = OutputPayload {
        words: result,
    };

    Ok(Json(output))
}
