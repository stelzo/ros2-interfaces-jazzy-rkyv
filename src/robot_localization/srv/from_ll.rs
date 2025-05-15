use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FromLLRequest {
    pub ll_point: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for FromLLRequest {
    fn default() -> Self {
        FromLLRequest {
            ll_point: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FromLLResponse {
    pub map_point: crate::geometry_msgs::msg::Point,
}

impl Default for FromLLResponse {
    fn default() -> Self {
        FromLLResponse {
            map_point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

pub struct FromLL;
