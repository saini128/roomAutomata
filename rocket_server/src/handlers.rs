use rocket::serde::json::{json, Value};
use rocket::State;

use crate::data::MyState;

// Simple handler for the root path
#[get("/")]
pub fn index() -> &'static str {
    "Welcome to roomAutomata Remote Server. This is a property of SinghRopar Inc."
}

// Handler to get the current state
#[get("/data")]
pub fn get_data(state: &State<MyState>) -> Value {
    let data = state.data.lock().unwrap();
    json!({ "message": "Here is the data", "value": *data })
}

// Handler to update the shared state
#[post("/data/<new_value>")]
pub fn update_data(state: &State<MyState>, new_value: String) -> Value {
    let mut data = state.data.lock().unwrap();
    *data = new_value;
    json!({ "message": "Data updated successfully" })
}
