use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HotStartRequest {
    pub reset_type: u8,
}

impl Default for HotStartRequest {
    fn default() -> Self {
        HotStartRequest { reset_type: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HotStartResponse {}

impl Default for HotStartResponse {
    fn default() -> Self {
        HotStartResponse {}
    }
}

pub struct HotStart;
