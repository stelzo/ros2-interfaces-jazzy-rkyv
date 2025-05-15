use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRobotStateFromWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for GetRobotStateFromWarehouseRequest {
    fn default() -> Self {
        GetRobotStateFromWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRobotStateFromWarehouseResponse {
    pub state: crate::moveit_msgs::msg::RobotState,
}

impl Default for GetRobotStateFromWarehouseResponse {
    fn default() -> Self {
        GetRobotStateFromWarehouseResponse {
            state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

pub struct GetRobotStateFromWarehouse;
