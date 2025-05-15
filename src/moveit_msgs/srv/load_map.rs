use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadMapRequest {
    pub filename: ::std::string::String,
}

impl Default for LoadMapRequest {
    fn default() -> Self {
        LoadMapRequest {
            filename: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadMapResponse {
    pub success: bool,
}

impl Default for LoadMapResponse {
    fn default() -> Self {
        LoadMapResponse { success: false }
    }
}

pub struct LoadMap;
