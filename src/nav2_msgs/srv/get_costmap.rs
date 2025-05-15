use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetCostmapRequest {
    pub specs: crate::nav2_msgs::msg::CostmapMetaData,
}

impl Default for GetCostmapRequest {
    fn default() -> Self {
        GetCostmapRequest {
            specs: crate::nav2_msgs::msg::CostmapMetaData::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetCostmapResponse {
    pub map: crate::nav2_msgs::msg::Costmap,
}

impl Default for GetCostmapResponse {
    fn default() -> Self {
        GetCostmapResponse {
            map: crate::nav2_msgs::msg::Costmap::default(),
        }
    }
}

pub struct GetCostmap;
