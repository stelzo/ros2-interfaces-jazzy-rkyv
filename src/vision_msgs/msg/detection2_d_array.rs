use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Detection2DArray {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::vision_msgs::msg::Detection2D>,
}

impl Default for Detection2DArray {
    fn default() -> Self {
        Detection2DArray {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}
