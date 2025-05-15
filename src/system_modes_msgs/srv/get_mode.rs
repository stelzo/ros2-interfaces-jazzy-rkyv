use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModeRequest {}

impl Default for GetModeRequest {
    fn default() -> Self {
        GetModeRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModeResponse {
    pub current_mode: ::std::string::String,
}

impl Default for GetModeResponse {
    fn default() -> Self {
        GetModeResponse {
            current_mode: ::std::string::String::new(),
        }
    }
}

pub struct GetMode;
