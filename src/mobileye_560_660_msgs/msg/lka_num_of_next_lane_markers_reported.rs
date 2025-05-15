use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LkaNumOfNextLaneMarkersReported {
    pub header: crate::std_msgs::msg::Header,
    pub num_of_next_lane_markers_reported: u16,
}

impl Default for LkaNumOfNextLaneMarkersReported {
    fn default() -> Self {
        LkaNumOfNextLaneMarkersReported {
            header: crate::std_msgs::msg::Header::default(),
            num_of_next_lane_markers_reported: 0,
        }
    }
}
