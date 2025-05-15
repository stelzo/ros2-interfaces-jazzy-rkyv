use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PoseWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
}

impl Default for PoseWithCovarianceStamped {
    fn default() -> Self {
        PoseWithCovarianceStamped {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
        }
    }
}
