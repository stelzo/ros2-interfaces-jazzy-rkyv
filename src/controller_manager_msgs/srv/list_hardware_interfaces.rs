use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListHardwareInterfacesRequest {}

impl Default for ListHardwareInterfacesRequest {
    fn default() -> Self {
        ListHardwareInterfacesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListHardwareInterfacesResponse {
    pub command_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
    pub state_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
}

impl Default for ListHardwareInterfacesResponse {
    fn default() -> Self {
        ListHardwareInterfacesResponse {
            command_interfaces: Vec::new(),
            state_interfaces: Vec::new(),
        }
    }
}

pub struct ListHardwareInterfaces;
