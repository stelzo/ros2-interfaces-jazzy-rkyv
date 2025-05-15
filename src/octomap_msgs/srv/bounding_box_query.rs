use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BoundingBoxQueryRequest {
    pub min: crate::geometry_msgs::msg::Point,
    pub max: crate::geometry_msgs::msg::Point,
}

impl Default for BoundingBoxQueryRequest {
    fn default() -> Self {
        BoundingBoxQueryRequest {
            min: crate::geometry_msgs::msg::Point::default(),
            max: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BoundingBoxQueryResponse {}

impl Default for BoundingBoxQueryResponse {
    fn default() -> Self {
        BoundingBoxQueryResponse {}
    }
}

pub struct BoundingBoxQuery;
