use std::sync::Mutex;

pub struct MyState {
    pub data: Mutex<String>,
}

impl Default for MyState {
    fn default() -> Self {
        MyState {
            data: Mutex::new("Initial Value".to_string()),
        }
    }
}

