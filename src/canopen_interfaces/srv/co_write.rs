use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COWriteRequest {
    pub index: u16,
    pub subindex: u8,
    pub data: u32,
}

impl Default for COWriteRequest {
    fn default() -> Self {
        COWriteRequest {
            index: 0,
            subindex: 0,
            data: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COWriteResponse {
    pub success: bool,
}

impl Default for COWriteResponse {
    fn default() -> Self {
        COWriteResponse { success: false }
    }
}

pub struct COWrite;
