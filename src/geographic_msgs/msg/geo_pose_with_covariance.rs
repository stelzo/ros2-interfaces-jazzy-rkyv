use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GeoPoseWithCovariance {
    pub pose: crate::geographic_msgs::msg::GeoPose,
    pub covariance: [f64; 36],
}

impl Default for GeoPoseWithCovariance {
    fn default() -> Self {
        GeoPoseWithCovariance {
            pose: crate::geographic_msgs::msg::GeoPose::default(),
            covariance: [0.0; 36],
        }
    }
}
