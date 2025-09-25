use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Status, ContentType},
    Data, Request, Response,
};
use rocket::serde::json::json;
use std::env;
use std::io::Cursor;

pub struct ApiKeyFairing;

#[rocket::async_trait]
impl Fairing for ApiKeyFairing {
    fn info(&self) -> Info {
        Info {
            name: "API Key Validator",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        // Skip root/index so you can have a welcome message without auth
        if req.uri().path() == "/" {
            return;
        }

        let provided = req.headers().get_one("x-api-key");
        let expected = env::var("API_KEY").unwrap_or_default();

        if provided != Some(expected.as_str()) {
            // Store failure flag in request's local cache
            req.local_cache(|| false);
        } else {
            req.local_cache(|| true);
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        // If request was invalid, override the response
        let valid = req.local_cache(|| true); // default true
        if !*valid {
            let body = json!({ "error": "Unauthorized or missing API key" }).to_string();
            res.set_status(Status::Unauthorized);
            res.set_header(ContentType::JSON);
            res.set_sized_body(body.len(), Cursor::new(body));
        }
    }
}
