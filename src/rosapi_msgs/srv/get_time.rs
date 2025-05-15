use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetTimeRequest {}

impl Default for GetTimeRequest {
    fn default() -> Self {
        GetTimeRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetTimeResponse {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for GetTimeResponse {
    fn default() -> Self {
        GetTimeResponse {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

pub struct GetTime;
