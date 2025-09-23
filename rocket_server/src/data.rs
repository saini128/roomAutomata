use std::sync::Mutex;
use crate::models::Room;
pub struct MyState {
    pub data: Mutex<String>,
    pub room: Mutex<Room>,
}

impl Default for MyState {
    fn default() -> Self {
        MyState {
            data: Mutex::new("Initial Value".to_string()),
            room: Mutex::new(Room::default()),
        }
    }
}

