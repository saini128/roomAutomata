use rocket::response::{status, stream::{Event, EventStream}};
use rocket::tokio::time::{interval, Duration};

use rocket::serde::json::{Json, json, Value};

use rocket::State;
use crate::models::Room;

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
    Ok(json!(*data))
}

// Handler to update the shared state
#[post("/update_state", data = "<new_value>")]
pub fn update_data(state: &State<MyState>, new_value: Json<Room>) -> status::Accepted<Value> {
    let mut data = state.room.lock().unwrap();
    *data = new_value.into_inner();
    status::Accepted(json!({ "message": "Data updated successfully" }))
}

#[get("/ping?<n>")]
pub fn stream(state: &State<MyState>, n: Option<u64>) -> EventStream![] {
    // Move only the Arc into the async block
    let room_arc = state.room.clone();

    EventStream! {
        let mut timer = interval(Duration::from_millis(n.unwrap_or(1000)));

        loop {
            // Lock only briefly and clone the inner Room
            let snapshot = {
                let data = room_arc.lock().unwrap();
                data.clone() // clone inner Room
            }; // <- mutex unlocked here

            let json_data = serde_json::to_string(&snapshot).unwrap();
            yield Event::data(json_data);

            timer.tick().await;
        }
    }
}

