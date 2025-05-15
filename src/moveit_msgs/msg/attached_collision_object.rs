use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AttachedCollisionObject {
    pub link_name: ::std::string::String,
    pub object: crate::moveit_msgs::msg::CollisionObject,
    pub touch_links: Vec<::std::string::String>,
    pub detach_posture: crate::trajectory_msgs::msg::JointTrajectory,
    pub weight: f64,
}

impl Default for AttachedCollisionObject {
    fn default() -> Self {
        AttachedCollisionObject {
            link_name: ::std::string::String::new(),
            object: crate::moveit_msgs::msg::CollisionObject::default(),
            touch_links: Vec::new(),
            detach_posture: crate::trajectory_msgs::msg::JointTrajectory::default(),
            weight: 0.0,
        }
    }
}
