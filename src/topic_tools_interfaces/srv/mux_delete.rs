use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxDeleteRequest {
    pub topic: ::std::string::String,
}

impl Default for MuxDeleteRequest {
    fn default() -> Self {
        MuxDeleteRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxDeleteResponse {
    pub success: bool,
}

impl Default for MuxDeleteResponse {
    fn default() -> Self {
        MuxDeleteResponse { success: false }
    }
}

pub struct MuxDelete;
