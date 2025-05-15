use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DqnRequest {
    pub action: u8,
    pub init: bool,
}

impl Default for DqnRequest {
    fn default() -> Self {
        DqnRequest {
            action: 0,
            init: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DqnResponse {
    pub state: Vec<f32>,
    pub reward: f32,
    pub done: bool,
}

impl Default for DqnResponse {
    fn default() -> Self {
        DqnResponse {
            state: Vec::new(),
            reward: 0.0,
            done: false,
        }
    }
}

pub struct Dqn;
