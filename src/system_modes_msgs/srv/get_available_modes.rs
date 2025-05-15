use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetAvailableModesRequest {}

impl Default for GetAvailableModesRequest {
    fn default() -> Self {
        GetAvailableModesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetAvailableModesResponse {
    pub available_modes: Vec<::std::string::String>,
}

impl Default for GetAvailableModesResponse {
    fn default() -> Self {
        GetAvailableModesResponse {
            available_modes: Vec::new(),
        }
    }
}

pub struct GetAvailableModes;
