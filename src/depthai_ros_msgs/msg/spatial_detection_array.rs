use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpatialDetectionArray {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::depthai_ros_msgs::msg::SpatialDetection>,
}

impl Default for SpatialDetectionArray {
    fn default() -> Self {
        SpatialDetectionArray {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}
