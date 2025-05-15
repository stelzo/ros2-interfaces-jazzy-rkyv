use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListRobotStatesInWarehouseRequest {
    pub regex: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for ListRobotStatesInWarehouseRequest {
    fn default() -> Self {
        ListRobotStatesInWarehouseRequest {
            regex: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListRobotStatesInWarehouseResponse {
    pub states: Vec<::std::string::String>,
}

impl Default for ListRobotStatesInWarehouseResponse {
    fn default() -> Self {
        ListRobotStatesInWarehouseResponse { states: Vec::new() }
    }
}

pub struct ListRobotStatesInWarehouse;
