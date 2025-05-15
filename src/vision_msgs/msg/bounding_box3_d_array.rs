use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BoundingBox3DArray {
    pub header: crate::std_msgs::msg::Header,
    pub boxes: Vec<crate::vision_msgs::msg::BoundingBox3D>,
}

impl Default for BoundingBox3DArray {
    fn default() -> Self {
        BoundingBox3DArray {
            header: crate::std_msgs::msg::Header::default(),
            boxes: Vec::new(),
        }
    }
}
