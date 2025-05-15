use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControlWorldRequest {
    pub world_control: crate::ros_gz_interfaces::msg::WorldControl,
}

impl Default for ControlWorldRequest {
    fn default() -> Self {
        ControlWorldRequest {
            world_control: crate::ros_gz_interfaces::msg::WorldControl::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControlWorldResponse {
    pub success: bool,
}

impl Default for ControlWorldResponse {
    fn default() -> Self {
        ControlWorldResponse { success: false }
    }
}

pub struct ControlWorld;
