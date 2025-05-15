use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BoundingBox {
    pub min_pt: crate::geographic_msgs::msg::GeoPoint,
    pub max_pt: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            min_pt: crate::geographic_msgs::msg::GeoPoint::default(),
            max_pt: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}
