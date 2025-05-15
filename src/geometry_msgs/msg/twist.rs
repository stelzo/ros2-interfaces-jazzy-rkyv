use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Twist {
    pub linear: crate::geometry_msgs::msg::Vector3,
    pub angular: crate::geometry_msgs::msg::Vector3,
}

impl Default for Twist {
    fn default() -> Self {
        Twist {
            linear: crate::geometry_msgs::msg::Vector3::default(),
            angular: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}
