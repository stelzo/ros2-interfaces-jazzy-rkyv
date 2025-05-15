use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WayPoint {
    pub id: crate::unique_identifier_msgs::msg::UUID,
    pub position: crate::geographic_msgs::msg::GeoPoint,
    pub props: Vec<crate::geographic_msgs::msg::KeyValue>,
}

impl Default for WayPoint {
    fn default() -> Self {
        WayPoint {
            id: crate::unique_identifier_msgs::msg::UUID::default(),
            position: crate::geographic_msgs::msg::GeoPoint::default(),
            props: Vec::new(),
        }
    }
}
