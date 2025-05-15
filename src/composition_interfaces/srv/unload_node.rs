use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UnloadNodeRequest {
    pub unique_id: u64,
}

impl Default for UnloadNodeRequest {
    fn default() -> Self {
        UnloadNodeRequest { unique_id: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UnloadNodeResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for UnloadNodeResponse {
    fn default() -> Self {
        UnloadNodeResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

pub struct UnloadNode;
