use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EStopRequest {
    pub e_stop_on: bool,
}

impl Default for EStopRequest {
    fn default() -> Self {
        EStopRequest { e_stop_on: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EStopResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for EStopResponse {
    fn default() -> Self {
        EStopResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct EStop;
