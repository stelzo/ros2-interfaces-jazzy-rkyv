use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveMapRequest {
    pub filename: ::std::string::String,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            filename: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveMapResponse {
    pub success: bool,
}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse { success: false }
    }
}

pub struct SaveMap;
