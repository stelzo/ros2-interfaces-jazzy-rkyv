use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TwistWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub twist: crate::geometry_msgs::msg::TwistWithCovariance,
}

impl Default for TwistWithCovarianceStamped {
    fn default() -> Self {
        TwistWithCovarianceStamped {
            header: crate::std_msgs::msg::Header::default(),
            twist: crate::geometry_msgs::msg::TwistWithCovariance::default(),
        }
    }
}
