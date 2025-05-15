use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRobotStateFromWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for DeleteRobotStateFromWarehouseRequest {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRobotStateFromWarehouseResponse {}

impl Default for DeleteRobotStateFromWarehouseResponse {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseResponse {}
    }
}

pub struct DeleteRobotStateFromWarehouse;
