use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListControllersRequest {}

impl Default for ListControllersRequest {
    fn default() -> Self {
        ListControllersRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListControllersResponse {
    pub controller: Vec<crate::controller_manager_msgs::msg::ControllerState>,
}

impl Default for ListControllersResponse {
    fn default() -> Self {
        ListControllersResponse {
            controller: Vec::new(),
        }
    }
}

pub struct ListControllers;
