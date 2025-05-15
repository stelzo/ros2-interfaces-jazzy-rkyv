use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObjectArray {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::derived_object_msgs::msg::Object>,
}

impl Default for ObjectArray {
    fn default() -> Self {
        ObjectArray {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}
