use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetBoolRequest {
    pub data: bool,
}

impl Default for SetBoolRequest {
    fn default() -> Self {
        SetBoolRequest { data: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetBoolResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetBoolResponse {
    fn default() -> Self {
        SetBoolResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SetBool;
