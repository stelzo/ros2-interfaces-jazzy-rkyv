use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DetectedObjects {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::autoware_perception_msgs::msg::DetectedObject>,
}

impl Default for DetectedObjects {
    fn default() -> Self {
        DetectedObjects {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}
