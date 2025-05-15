use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetStringRequest {
    pub value: ::std::string::String,
}

impl Default for SetStringRequest {
    fn default() -> Self {
        SetStringRequest {
            value: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetStringResponse {
    pub success: bool,
}

impl Default for SetStringResponse {
    fn default() -> Self {
        SetStringResponse { success: false }
    }
}

pub struct SetString;
