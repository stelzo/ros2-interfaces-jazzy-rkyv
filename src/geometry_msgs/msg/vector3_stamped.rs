use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Vector3Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub vector: crate::geometry_msgs::msg::Vector3,
}

impl Default for Vector3Stamped {
    fn default() -> Self {
        Vector3Stamped {
            header: crate::std_msgs::msg::Header::default(),
            vector: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}
