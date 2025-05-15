use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxAddRequest {
    pub topic: ::std::string::String,
}

impl Default for MuxAddRequest {
    fn default() -> Self {
        MuxAddRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxAddResponse {
    pub success: bool,
}

impl Default for MuxAddResponse {
    fn default() -> Self {
        MuxAddResponse { success: false }
    }
}

pub struct MuxAdd;
