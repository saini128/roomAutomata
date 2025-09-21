use rocket::serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RoomState {
    pub id: String, 
    pub name: String,
    pub state: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeviceState {
    pub id: String, // `Option<u32>` allows for a nullable ID, useful for new users
    pub name: String,
    pub state: String,
}