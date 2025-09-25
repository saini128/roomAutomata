use rocket::{http::Status, request::{FromRequest, Outcome}, Request};
use std::env;

pub struct ApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        // Extract "x-api-key" header
        match req.headers().get_one("x-api-key") {
            Some(key) => {
                let expected = env::var("API_KEY").unwrap_or_default();
                if key == expected {
                    Outcome::Success(ApiKey)
                } else {
                    Outcome::Error((Status::Unauthorized, ()))
                }
            }
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
