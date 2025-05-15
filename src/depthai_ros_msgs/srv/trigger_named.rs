use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TriggerNamedRequest {
    pub name: ::std::string::String,
}

impl Default for TriggerNamedRequest {
    fn default() -> Self {
        TriggerNamedRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TriggerNamedResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerNamedResponse {
    fn default() -> Self {
        TriggerNamedResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct TriggerNamed;
