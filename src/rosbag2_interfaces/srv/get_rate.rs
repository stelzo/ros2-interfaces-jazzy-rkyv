use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRateRequest {}

impl Default for GetRateRequest {
    fn default() -> Self {
        GetRateRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRateResponse {
    pub rate: f64,
}

impl Default for GetRateResponse {
    fn default() -> Self {
        GetRateResponse { rate: 0.0 }
    }
}

pub struct GetRate;
