use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveMapRequest {
    pub filename: crate::std_msgs::msg::String,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            filename: crate::std_msgs::msg::String::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveMapResponse {}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse {}
    }
}

pub struct SaveMap;
