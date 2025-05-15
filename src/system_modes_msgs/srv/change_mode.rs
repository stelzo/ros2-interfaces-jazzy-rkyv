use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChangeModeRequest {
    pub mode_name: ::std::string::String,
}

impl Default for ChangeModeRequest {
    fn default() -> Self {
        ChangeModeRequest {
            mode_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChangeModeResponse {
    pub success: bool,
}

impl Default for ChangeModeResponse {
    fn default() -> Self {
        ChangeModeResponse { success: false }
    }
}

pub struct ChangeMode;
