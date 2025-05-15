use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ToLLRequest {
    pub map_point: crate::geometry_msgs::msg::Point,
}

impl Default for ToLLRequest {
    fn default() -> Self {
        ToLLRequest {
            map_point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ToLLResponse {
    pub ll_point: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for ToLLResponse {
    fn default() -> Self {
        ToLLResponse {
            ll_point: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

pub struct ToLL;
