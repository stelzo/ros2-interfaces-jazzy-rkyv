use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearCostmapExceptRegionRequest {
    pub reset_distance: f32,
}

impl Default for ClearCostmapExceptRegionRequest {
    fn default() -> Self {
        ClearCostmapExceptRegionRequest {
            reset_distance: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearCostmapExceptRegionResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearCostmapExceptRegionResponse {
    fn default() -> Self {
        ClearCostmapExceptRegionResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

pub struct ClearCostmapExceptRegion;
