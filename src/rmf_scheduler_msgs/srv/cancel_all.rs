use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelAllRequest {
    pub group: ::std::string::String,
}

impl Default for CancelAllRequest {
    fn default() -> Self {
        CancelAllRequest {
            group: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelAllResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelAllResponse {
    fn default() -> Self {
        CancelAllResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct CancelAll;
