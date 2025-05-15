use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RenameRobotStateInWarehouseRequest {
    pub old_name: ::std::string::String,
    pub new_name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for RenameRobotStateInWarehouseRequest {
    fn default() -> Self {
        RenameRobotStateInWarehouseRequest {
            old_name: ::std::string::String::new(),
            new_name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RenameRobotStateInWarehouseResponse {}

impl Default for RenameRobotStateInWarehouseResponse {
    fn default() -> Self {
        RenameRobotStateInWarehouseResponse {}
    }
}

pub struct RenameRobotStateInWarehouse;
