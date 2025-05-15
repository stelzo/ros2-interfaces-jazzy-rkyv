use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetCostRequest {
    pub use_footprint: bool,
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Default for GetCostRequest {
    fn default() -> Self {
        GetCostRequest {
            use_footprint: false,
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetCostResponse {
    pub cost: f32,
}

impl Default for GetCostResponse {
    fn default() -> Self {
        GetCostResponse { cost: 0.0 }
    }
}

pub struct GetCost;
