use std::sync::Mutex;

// This struct holds the shared application state.
// We use `Mutex` to allow for safe, mutable access across threads.
// It's managed by Rocket and can be retrieved in handlers with `State<MyState>`.
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
