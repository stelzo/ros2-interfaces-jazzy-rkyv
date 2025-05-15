use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct VirtualGateStatus {
    pub areas: Vec<crate::autoware_v2x_msgs::msg::VirtualGateAreaStatus>,
}

impl Default for VirtualGateStatus {
    fn default() -> Self {
        VirtualGateStatus { areas: Vec::new() }
    }
}
