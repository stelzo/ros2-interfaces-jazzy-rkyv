use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObjectWithCovarianceArray {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::derived_object_msgs::msg::ObjectWithCovariance>,
}

impl Default for ObjectWithCovarianceArray {
    fn default() -> Self {
        ObjectWithCovarianceArray {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}
