use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ToggleFilterProcessingRequest {
    pub on: bool,
}

impl Default for ToggleFilterProcessingRequest {
    fn default() -> Self {
        ToggleFilterProcessingRequest { on: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ToggleFilterProcessingResponse {
    pub status: bool,
}

impl Default for ToggleFilterProcessingResponse {
    fn default() -> Self {
        ToggleFilterProcessingResponse { status: false }
    }
}

pub struct ToggleFilterProcessing;
