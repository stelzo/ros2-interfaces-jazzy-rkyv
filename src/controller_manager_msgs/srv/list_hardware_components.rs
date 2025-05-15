use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListHardwareComponentsRequest {}

impl Default for ListHardwareComponentsRequest {
    fn default() -> Self {
        ListHardwareComponentsRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListHardwareComponentsResponse {
    pub component: Vec<crate::controller_manager_msgs::msg::HardwareComponentState>,
}

impl Default for ListHardwareComponentsResponse {
    fn default() -> Self {
        ListHardwareComponentsResponse {
            component: Vec::new(),
        }
    }
}

pub struct ListHardwareComponents;
