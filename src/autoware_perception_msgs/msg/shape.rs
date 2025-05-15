use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Shape {
    pub r#type: u8,
    pub footprint: crate::geometry_msgs::msg::Polygon,
    pub dimensions: crate::geometry_msgs::msg::Vector3,
}

impl Shape {
    pub const BOUNDING_BOX: u8 = 0;
    pub const CYLINDER: u8 = 1;
    pub const POLYGON: u8 = 2;
}

impl Default for Shape {
    fn default() -> Self {
        Shape {
            r#type: 0,
            footprint: crate::geometry_msgs::msg::Polygon::default(),
            dimensions: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}
