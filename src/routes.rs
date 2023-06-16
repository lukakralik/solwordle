use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::utils::*;
use crate::search::*;
use crate::models::{InputPayload, OutputPayload};

#[post("/words", format = "json", data = "<input>")]
pub fn fitting_sequence(input: Json<InputPayload>) -> Result<Json<OutputPayload>, Status> {

    let data = import_dataset(&input.lang.clone(), input.length.clone());
    
    let result = filter_words(data, "---L----", &['E', 'A', 'O', 'P'], &['S', 'C']);
    println!("{:#?}", result);

    let output = OutputPayload {
        words: result,
    };

    Ok(Json(output))
}
