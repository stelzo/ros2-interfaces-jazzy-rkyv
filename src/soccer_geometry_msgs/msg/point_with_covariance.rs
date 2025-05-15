use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PointWithCovariance {
    pub point: crate::geometry_msgs::msg::Point,
    pub covariance: [f64; 9],
}

impl Default for PointWithCovariance {
    fn default() -> Self {
        PointWithCovariance {
            point: crate::geometry_msgs::msg::Point::default(),
            covariance: [0.0; 9],
        }
    }
}
