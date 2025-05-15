use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Detection3DArray {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::vision_msgs::msg::Detection3D>,
}

impl Default for Detection3DArray {
    fn default() -> Self {
        Detection3DArray {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}
