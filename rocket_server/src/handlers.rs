use rocket::response::status;
use rocket::serde::json::{json, Value};
use rocket::State;

use crate::data::MyState;


// -------------------- API Key Guard --------------------
// Simple handler for the root path
#[get("/")]
pub fn index() -> status::Accepted<Value> {
    status::Accepted(json!({"message": "Welcome to roomAutomata Remote Server. This is a property of SinghRopar Inc."}))
}

// Handler to get the current state
#[get("/status")]
pub fn get_data(state: &State<MyState>) -> Result<Value, status::Custom<Value>> {
    let data = state.room.lock().unwrap();
    Ok(json!({ "message": "Here is the device status", "value": *data }))
}

// Handler to update the shared state
#[post("/update_state", data = "<new_value>")]
pub fn update_data(state: &State<MyState>, new_value: String) -> status::Accepted<Value> {
    let mut data = state.data.lock().unwrap();
    *data = new_value;
    status::Accepted(json!({ "message": "Data updated successfully" }))
}
