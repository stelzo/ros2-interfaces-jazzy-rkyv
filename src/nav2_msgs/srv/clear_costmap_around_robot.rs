use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearCostmapAroundRobotRequest {
    pub reset_distance: f32,
}

impl Default for ClearCostmapAroundRobotRequest {
    fn default() -> Self {
        ClearCostmapAroundRobotRequest {
            reset_distance: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearCostmapAroundRobotResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearCostmapAroundRobotResponse {
    fn default() -> Self {
        ClearCostmapAroundRobotResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

pub struct ClearCostmapAroundRobot;
