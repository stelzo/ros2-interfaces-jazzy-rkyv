use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CheckIfRobotStateExistsInWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for CheckIfRobotStateExistsInWarehouseRequest {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CheckIfRobotStateExistsInWarehouseResponse {
    pub exists: bool,
}

impl Default for CheckIfRobotStateExistsInWarehouseResponse {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseResponse { exists: false }
    }
}

pub struct CheckIfRobotStateExistsInWarehouse;
