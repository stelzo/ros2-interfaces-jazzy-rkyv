use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRobotModeRequest {}

impl Default for GetRobotModeRequest {
    fn default() -> Self {
        GetRobotModeRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRobotModeResponse {
    pub robot_mode: crate::ur_dashboard_msgs::msg::RobotMode,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetRobotModeResponse {
    fn default() -> Self {
        GetRobotModeResponse {
            robot_mode: crate::ur_dashboard_msgs::msg::RobotMode::default(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct GetRobotMode;
