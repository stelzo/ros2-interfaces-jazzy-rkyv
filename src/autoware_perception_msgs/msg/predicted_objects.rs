use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PredictedObjects {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::autoware_perception_msgs::msg::PredictedObject>,
}

impl Default for PredictedObjects {
    fn default() -> Self {
        PredictedObjects {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}
