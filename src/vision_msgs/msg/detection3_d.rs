use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Detection3D {
    pub header: crate::std_msgs::msg::Header,
    pub results: Vec<crate::vision_msgs::msg::ObjectHypothesisWithPose>,
    pub bbox: crate::vision_msgs::msg::BoundingBox3D,
    pub id: ::std::string::String,
}

impl Default for Detection3D {
    fn default() -> Self {
        Detection3D {
            header: crate::std_msgs::msg::Header::default(),
            results: Vec::new(),
            bbox: crate::vision_msgs::msg::BoundingBox3D::default(),
            id: ::std::string::String::new(),
        }
    }
}
