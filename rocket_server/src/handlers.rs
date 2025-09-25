use rocket::{http, response::{status, stream::{Event, EventStream}}};
use rocket::tokio::time::{interval, Duration};

use rocket::serde::json::{Json, json, Value};

use rocket::State;
use crate::models::{UpdateRequest, Device, LightState, SwitchState, PartialAcState};
use crate::data::MyState;


// -------------------- API Key Guard --------------------
// Simple handler for the root path
#[get("/")]
pub fn index() -> status::Accepted<Value> {
    status::Accepted(json!({"message": "Welcome to roomAutomata Remote Server. This is a property of SinghRopar Inc."}))
}

// Handler to get the current state
#[get("/status")]
pub fn get_data( state: &State<MyState>) -> Result<Value, status::Custom<Value>> {
    let data = state.room.lock().unwrap();
    Ok(json!(*data))
}

#[post("/update_state", data = "<update_req>")]
pub fn update_data(
    state: &State<MyState>,
    update_req: Json<UpdateRequest>,
) -> Result<status::Accepted<Value>, status::Custom<Value>> {
    let mut room = state.room.lock().unwrap();
    let device_id = update_req.device_id;

    if let Some(device) = room.devices.iter_mut().find(|d| match d {
        Device::Ac { id, .. } => *id == device_id,
        Device::Light { id, .. } => *id == device_id,
        Device::Switch { id, .. } => *id == device_id,
    }) {
        match device {
            Device::Ac { state, .. } => {
                if let Ok(new_state) = serde_json::from_value::<PartialAcState>(update_req.state.clone()) {
                    if let Some(on) = new_state.on {
                        state.on = on;
                    }
                    if let Some(temp) = new_state.temperature {
                        state.temperature = temp;
                    }
                } else {
                    return Err(status::Custom(
                        http::Status::BadRequest,
                        json!({"error": "Invalid state for AC"}),
                    ));
                }
            }
            Device::Light { state, .. } => {
                if let Ok(new_state) = serde_json::from_value::<LightState>(update_req.state.clone()) {
                    *state = new_state;
                } else {
                    return Err(status::Custom(
                        http::Status::BadRequest,
                        json!({"error": "Invalid state for Light"}),
                    ));
                }
            }
            Device::Switch { state, .. } => {
                if let Ok(new_state) = serde_json::from_value::<SwitchState>(update_req.state.clone()) {
                    *state = new_state;
                } else {
                    return Err(status::Custom(
                        http::Status::BadRequest,
                        json!({"error": "Invalid state for Switch"}),
                    ));
                }
            }
        }

        Ok(status::Accepted(json!({"message": "Device state updated successfully"})))
    } else {
        Err(status::Custom(
            http::Status::NotFound,
            json!({"error": "Device not found"}),
        ))
    }
}

#[get("/status-sse?<n>")]
pub fn stream(state: &State<MyState>, n: Option<u64>) -> EventStream![] {
    // Move only the Arc into the async block
    let room_arc = state.room.clone();

    EventStream! {
        let mut timer = interval(Duration::from_millis(n.unwrap_or(100)));

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

