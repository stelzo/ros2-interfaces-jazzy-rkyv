use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Wrench {
    pub force: crate::geometry_msgs::msg::Vector3,
    pub torque: crate::geometry_msgs::msg::Vector3,
}

impl Default for Wrench {
    fn default() -> Self {
        Wrench {
            force: crate::geometry_msgs::msg::Vector3::default(),
            torque: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}
