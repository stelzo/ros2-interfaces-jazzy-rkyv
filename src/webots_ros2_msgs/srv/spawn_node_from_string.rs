use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnNodeFromStringRequest {
    pub data: ::std::string::String,
}

impl Default for SpawnNodeFromStringRequest {
    fn default() -> Self {
        SpawnNodeFromStringRequest {
            data: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnNodeFromStringResponse {
    pub success: bool,
}

impl Default for SpawnNodeFromStringResponse {
    fn default() -> Self {
        SpawnNodeFromStringResponse { success: false }
    }
}

pub struct SpawnNodeFromString;
