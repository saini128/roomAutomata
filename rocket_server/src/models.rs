
use rocket::serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    pub id: u32,
    pub devices: Vec<Device>,
}
impl Default for Room {
    fn default() -> Self {
        Room {
            id: 1,
            devices: vec![
                Device::Ac {
                    id: 1,
                    state: AcState{on: false, temperature: 24},
                },
                Device::Light {
                    id: 2,
                    state: LightState::Off,
                },
                Device::Switch {
                    id: 3,
                    pin: 5,
                    state: SwitchState::Off,
                },
            ],
        }
    }
}



#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
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


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AcState {
    pub on: bool,
    pub temperature: u32,
}


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

impl Default for Signal {
    fn default() -> Self {
        Signal::Static { pin: 0, value: false }
    }
}