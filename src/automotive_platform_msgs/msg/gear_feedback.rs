use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GearFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub current_gear: crate::automotive_platform_msgs::msg::Gear,
}

impl Default for GearFeedback {
    fn default() -> Self {
        GearFeedback {
            header: crate::std_msgs::msg::Header::default(),
            current_gear: crate::automotive_platform_msgs::msg::Gear::default(),
        }
    }
}
