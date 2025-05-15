use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetStateRequest {
    pub time_stamp: crate::builtin_interfaces::msg::Time,
    pub frame_id: ::std::string::String,
}

impl Default for GetStateRequest {
    fn default() -> Self {
        GetStateRequest {
            time_stamp: crate::builtin_interfaces::msg::Time::default(),
            frame_id: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetStateResponse {
    pub state: [f64; 15],
    pub covariance: [f64; 225],
}

impl Default for GetStateResponse {
    fn default() -> Self {
        GetStateResponse {
            state: [0.0; 15],
            covariance: [0.0; 225],
        }
    }
}

pub struct GetState;
