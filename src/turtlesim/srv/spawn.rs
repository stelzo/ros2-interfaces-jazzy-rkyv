use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnRequest {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub name: ::std::string::String,
}

impl Default for SpawnRequest {
    fn default() -> Self {
        SpawnRequest {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnResponse {
    pub name: ::std::string::String,
}

impl Default for SpawnResponse {
    fn default() -> Self {
        SpawnResponse {
            name: ::std::string::String::new(),
        }
    }
}

pub struct Spawn;
