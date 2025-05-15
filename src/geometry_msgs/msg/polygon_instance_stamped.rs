use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PolygonInstanceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub polygon: crate::geometry_msgs::msg::PolygonInstance,
}

impl Default for PolygonInstanceStamped {
    fn default() -> Self {
        PolygonInstanceStamped {
            header: crate::std_msgs::msg::Header::default(),
            polygon: crate::geometry_msgs::msg::PolygonInstance::default(),
        }
    }
}
