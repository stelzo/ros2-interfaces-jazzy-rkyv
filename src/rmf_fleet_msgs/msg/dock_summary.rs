use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DockSummary {
    pub docks: Vec<crate::rmf_fleet_msgs::msg::Dock>,
}

impl Default for DockSummary {
    fn default() -> Self {
        DockSummary { docks: Vec::new() }
    }
}
