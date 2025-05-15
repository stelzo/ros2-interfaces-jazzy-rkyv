use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LIDoutputstateSrvRequest {
    pub active: bool,
}

impl Default for LIDoutputstateSrvRequest {
    fn default() -> Self {
        LIDoutputstateSrvRequest { active: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LIDoutputstateSrvResponse {
    pub success: bool,
}

impl Default for LIDoutputstateSrvResponse {
    fn default() -> Self {
        LIDoutputstateSrvResponse { success: false }
    }
}

pub struct LIDoutputstateSrv;
