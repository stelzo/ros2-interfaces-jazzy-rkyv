use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetInteractiveMarkersRequest {}

impl Default for GetInteractiveMarkersRequest {
    fn default() -> Self {
        GetInteractiveMarkersRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetInteractiveMarkersResponse {
    pub sequence_number: u64,
    pub markers: Vec<crate::visualization_msgs::msg::InteractiveMarker>,
}

impl Default for GetInteractiveMarkersResponse {
    fn default() -> Self {
        GetInteractiveMarkersResponse {
            sequence_number: 0,
            markers: Vec::new(),
        }
    }
}

pub struct GetInteractiveMarkers;
