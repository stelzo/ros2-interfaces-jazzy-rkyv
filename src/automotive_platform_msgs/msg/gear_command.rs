use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GearCommand {
    pub header: crate::std_msgs::msg::Header,
    pub command: crate::automotive_platform_msgs::msg::Gear,
}

impl Default for GearCommand {
    fn default() -> Self {
        GearCommand {
            header: crate::std_msgs::msg::Header::default(),
            command: crate::automotive_platform_msgs::msg::Gear::default(),
        }
    }
}
