use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DockStatus {
    pub header: crate::std_msgs::msg::Header,
    pub dock_visible: bool,
    pub is_docked: bool,
}

impl Default for DockStatus {
    fn default() -> Self {
        DockStatus {
            header: crate::std_msgs::msg::Header::default(),
            dock_visible: false,
            is_docked: false,
        }
    }
}
