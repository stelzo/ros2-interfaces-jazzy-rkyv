use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WrenchStamped {
    pub header: crate::std_msgs::msg::Header,
    pub wrench: crate::geometry_msgs::msg::Wrench,
}

impl Default for WrenchStamped {
    fn default() -> Self {
        WrenchStamped {
            header: crate::std_msgs::msg::Header::default(),
            wrench: crate::geometry_msgs::msg::Wrench::default(),
        }
    }
}
