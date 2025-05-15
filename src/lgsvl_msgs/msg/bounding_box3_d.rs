use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BoundingBox3D {
    pub position: crate::geometry_msgs::msg::Pose,
    pub size: crate::geometry_msgs::msg::Vector3,
}

impl Default for BoundingBox3D {
    fn default() -> Self {
        BoundingBox3D {
            position: crate::geometry_msgs::msg::Pose::default(),
            size: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}
