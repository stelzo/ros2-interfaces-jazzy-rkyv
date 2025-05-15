use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetDatumRequest {
    pub geo_pose: crate::geographic_msgs::msg::GeoPose,
}

impl Default for SetDatumRequest {
    fn default() -> Self {
        SetDatumRequest {
            geo_pose: crate::geographic_msgs::msg::GeoPose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetDatumResponse {}

impl Default for SetDatumResponse {
    fn default() -> Self {
        SetDatumResponse {}
    }
}

pub struct SetDatum;
