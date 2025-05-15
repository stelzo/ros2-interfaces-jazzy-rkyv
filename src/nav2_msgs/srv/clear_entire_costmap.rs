use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearEntireCostmapRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapRequest {
    fn default() -> Self {
        ClearEntireCostmapRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearEntireCostmapResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapResponse {
    fn default() -> Self {
        ClearEntireCostmapResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

pub struct ClearEntireCostmap;
