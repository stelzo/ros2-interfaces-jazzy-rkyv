use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnUrdfRobotRequest {
    pub robot: crate::webots_ros2_msgs::msg::UrdfRobot,
}

impl Default for SpawnUrdfRobotRequest {
    fn default() -> Self {
        SpawnUrdfRobotRequest {
            robot: crate::webots_ros2_msgs::msg::UrdfRobot::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnUrdfRobotResponse {
    pub success: bool,
}

impl Default for SpawnUrdfRobotResponse {
    fn default() -> Self {
        SpawnUrdfRobotResponse { success: false }
    }
}

pub struct SpawnUrdfRobot;
