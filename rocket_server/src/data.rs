use std::sync::{Arc, Mutex};
use crate::models::Room;

pub struct MyState {
    pub room: Arc<Mutex<Room>>,
}

impl Default for MyState {
    fn default() -> Self {
        MyState {
            room: Arc::new(Mutex::new(Room::default())),
        }
    }
}
