use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLayersRequest {}

impl Default for GetLayersRequest {
    fn default() -> Self {
        GetLayersRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLayersResponse {
    pub layers: Vec<::std::string::String>,
}

impl Default for GetLayersResponse {
    fn default() -> Self {
        GetLayersResponse { layers: Vec::new() }
    }
}

pub struct GetLayers;
