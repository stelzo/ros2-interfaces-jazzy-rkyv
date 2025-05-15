use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PointStamped {
    pub header: crate::std_msgs::msg::Header,
    pub point: crate::geometry_msgs::msg::Point,
}

impl Default for PointStamped {
    fn default() -> Self {
        PointStamped {
            header: crate::std_msgs::msg::Header::default(),
            point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}
