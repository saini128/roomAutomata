#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

mod data;
mod handlers;
mod models;
mod middleware;
use data::MyState;
use dotenvy::dotenv;

// Define a CORS fairing to allow all origins and headers
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        // Attach the CORS fairing to allow all origins and headers
        .attach(CORS)
        // Manage the state for shared data
        .manage(MyState::default())
        // Mount all the handlers from the handlers module
        .mount("/", routes![handlers::index, handlers::get_data, handlers::update_data, handlers::stream])
}
