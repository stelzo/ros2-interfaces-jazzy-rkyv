use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HazardDetectionVector {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::irobot_create_msgs::msg::HazardDetection>,
}

impl Default for HazardDetectionVector {
    fn default() -> Self {
        HazardDetectionVector {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}
