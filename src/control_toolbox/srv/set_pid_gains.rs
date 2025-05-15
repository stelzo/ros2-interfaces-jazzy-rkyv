use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPidGainsRequest {
    pub p: f64,
    pub i: f64,
    pub d: f64,
    pub i_clamp: f64,
    pub antiwindup: bool,
}

impl Default for SetPidGainsRequest {
    fn default() -> Self {
        SetPidGainsRequest {
            p: 0.0,
            i: 0.0,
            d: 0.0,
            i_clamp: 0.0,
            antiwindup: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPidGainsResponse {}

impl Default for SetPidGainsResponse {
    fn default() -> Self {
        SetPidGainsResponse {}
    }
}

pub struct SetPidGains;
