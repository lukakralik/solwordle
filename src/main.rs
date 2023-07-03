#[cfg(feature = "server")]
mod server {
    #![feature(decl_macro)]
    #[macro_use]
    extern crate rocket;
    extern crate rocket_cors;

    use rocket::ignite;
    use rocket_cors::{AllowedOrigins, CorsOptions};
    use std::env::current_dir;
}

#[cfg(not(feature = "server"))]
mod cli {
    {}
}

pub use crate::utils::*;
pub use crate::search::*;
pub mod utils;
pub mod search;
pub mod routes;
pub mod models;

fn main() {
    #[cfg(feature = "server")] {
        println!("{:#?}", current_dir());
        let cors = CorsOptions::default()
            .allowed_origins(AllowedOrigins::all())
            .to_cors()
            .expect("Failed to initialize CORS");

        ignite()
            .mount("/", routes![routes::fitting_sequence])
            .attach(cors)
            .launch();
    }

    #[cfg(feature = "cli")] {
        {}
    }
}
