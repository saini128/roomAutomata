
use rocket::serde::{Deserialize, Serialize};

// The `Room` struct contains a collection of smart devices.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    pub id: u32,
    pub devices: Vec<Device>,
}

// The `Device` enum represents the different types of devices in a room.
// Each variant holds the specific data and state for that device type.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type", content = "details")]
pub enum Device {
    #[serde(rename = "ac")]
    Ac {
        id: u32,
        state: AcState,
    },
    #[serde(rename = "light")]
    Light {
        id: u32,
        state: LightState,
    },
    #[serde(rename = "switch")]
    Switch {
        id: u32,
        pin: u32,
        state: SwitchState,
    },
}

// The `AcState` struct holds the state information for an air conditioner.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AcState {
    pub on: bool,
    pub temperature: u32,
}

// The `LightState` enum handles the different modes for a light.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum LightState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "auto")]
    Auto,
}

// The `SwitchState` enum represents the on/off state of a switch.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum SwitchState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum Signal {
    #[serde(rename = "ir")]
    Ir {
        data: u64,
    },
    #[serde(rename = "static")]
    Static {
        pin: u32,
        value: bool,
    },
    #[serde(rename = "auto_light")]
    AutoLight {
        value: bool,
    },
}
