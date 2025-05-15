use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SeekRequest {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for SeekRequest {
    fn default() -> Self {
        SeekRequest {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SeekResponse {
    pub success: bool,
}

impl Default for SeekResponse {
    fn default() -> Self {
        SeekResponse { success: false }
    }
}

pub struct Seek;
