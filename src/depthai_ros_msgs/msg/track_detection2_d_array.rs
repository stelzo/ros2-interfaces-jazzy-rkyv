use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrackDetection2DArray {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::depthai_ros_msgs::msg::TrackDetection2D>,
}

impl Default for TrackDetection2DArray {
    fn default() -> Self {
        TrackDetection2DArray {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}
