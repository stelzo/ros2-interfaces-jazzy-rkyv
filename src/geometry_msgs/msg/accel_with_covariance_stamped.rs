use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AccelWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub accel: crate::geometry_msgs::msg::AccelWithCovariance,
}

impl Default for AccelWithCovarianceStamped {
    fn default() -> Self {
        AccelWithCovarianceStamped {
            header: crate::std_msgs::msg::Header::default(),
            accel: crate::geometry_msgs::msg::AccelWithCovariance::default(),
        }
    }
}
