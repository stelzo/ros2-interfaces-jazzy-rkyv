use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AccelWithCovariance {
    pub accel: crate::geometry_msgs::msg::Accel,
    pub covariance: [f64; 36],
}

impl Default for AccelWithCovariance {
    fn default() -> Self {
        AccelWithCovariance {
            accel: crate::geometry_msgs::msg::Accel::default(),
            covariance: [0.0; 36],
        }
    }
}
