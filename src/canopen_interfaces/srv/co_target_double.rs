use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COTargetDoubleRequest {
    pub target: f64,
}

impl Default for COTargetDoubleRequest {
    fn default() -> Self {
        COTargetDoubleRequest { target: 0.0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COTargetDoubleResponse {
    pub success: bool,
}

impl Default for COTargetDoubleResponse {
    fn default() -> Self {
        COTargetDoubleResponse { success: false }
    }
}

pub struct COTargetDouble;
