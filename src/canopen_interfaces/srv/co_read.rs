use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COReadRequest {
    pub index: u16,
    pub subindex: u8,
}

impl Default for COReadRequest {
    fn default() -> Self {
        COReadRequest {
            index: 0,
            subindex: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COReadResponse {
    pub success: bool,
    pub data: u32,
}

impl Default for COReadResponse {
    fn default() -> Self {
        COReadResponse {
            success: false,
            data: 0,
        }
    }
}

pub struct CORead;
