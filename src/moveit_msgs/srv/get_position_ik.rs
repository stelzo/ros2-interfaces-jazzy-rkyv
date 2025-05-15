use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPositionIKRequest {
    pub ik_request: crate::moveit_msgs::msg::PositionIKRequest,
}

impl Default for GetPositionIKRequest {
    fn default() -> Self {
        GetPositionIKRequest {
            ik_request: crate::moveit_msgs::msg::PositionIKRequest::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPositionIKResponse {
    pub solution: crate::moveit_msgs::msg::RobotState,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GetPositionIKResponse {
    fn default() -> Self {
        GetPositionIKResponse {
            solution: crate::moveit_msgs::msg::RobotState::default(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

pub struct GetPositionIK;
