use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BoundingBox2DArray {
    pub header: crate::std_msgs::msg::Header,
    pub boxes: Vec<crate::vision_msgs::msg::BoundingBox2D>,
}

impl Default for BoundingBox2DArray {
    fn default() -> Self {
        BoundingBox2DArray {
            header: crate::std_msgs::msg::Header::default(),
            boxes: Vec::new(),
        }
    }
}
