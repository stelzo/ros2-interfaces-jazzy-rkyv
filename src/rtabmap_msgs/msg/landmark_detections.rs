use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LandmarkDetections {
    pub header: crate::std_msgs::msg::Header,
    pub landmarks: Vec<crate::rtabmap_msgs::msg::LandmarkDetection>,
}

impl Default for LandmarkDetections {
    fn default() -> Self {
        LandmarkDetections {
            header: crate::std_msgs::msg::Header::default(),
            landmarks: Vec::new(),
        }
    }
}
