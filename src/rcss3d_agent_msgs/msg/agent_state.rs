use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AgentState {
    pub temp: f32,
    pub battery: f32,
}

impl Default for AgentState {
    fn default() -> Self {
        AgentState {
            temp: 0.0,
            battery: 0.0,
        }
    }
}
