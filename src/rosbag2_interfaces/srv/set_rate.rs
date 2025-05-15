use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRateRequest {
    pub rate: f64,
}

impl Default for SetRateRequest {
    fn default() -> Self {
        SetRateRequest { rate: 0.0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRateResponse {
    pub success: bool,
}

impl Default for SetRateResponse {
    fn default() -> Self {
        SetRateResponse { success: false }
    }
}

pub struct SetRate;
