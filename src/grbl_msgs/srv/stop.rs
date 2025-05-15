use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StopRequest {
    pub machine_id: ::std::string::String,
    pub command: ::std::string::String,
}

impl Default for StopRequest {
    fn default() -> Self {
        StopRequest {
            machine_id: ::std::string::String::new(),
            command: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StopResponse {
    pub success: bool,
}

impl Default for StopResponse {
    fn default() -> Self {
        StopResponse { success: false }
    }
}

pub struct Stop;
