use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Accelerometer {
    pub name: ::std::string::String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Accelerometer {
    fn default() -> Self {
        Accelerometer {
            name: ::std::string::String::new(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
