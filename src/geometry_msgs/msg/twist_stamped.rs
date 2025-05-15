use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TwistStamped {
    pub header: crate::std_msgs::msg::Header,
    pub twist: crate::geometry_msgs::msg::Twist,
}

impl Default for TwistStamped {
    fn default() -> Self {
        TwistStamped {
            header: crate::std_msgs::msg::Header::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}
