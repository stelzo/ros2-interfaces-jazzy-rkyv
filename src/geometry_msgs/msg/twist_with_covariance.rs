use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TwistWithCovariance {
    pub twist: crate::geometry_msgs::msg::Twist,
    pub covariance: [f64; 36],
}

impl Default for TwistWithCovariance {
    fn default() -> Self {
        TwistWithCovariance {
            twist: crate::geometry_msgs::msg::Twist::default(),
            covariance: [0.0; 36],
        }
    }
}
