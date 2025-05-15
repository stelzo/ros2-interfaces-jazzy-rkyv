use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GraspableObject {
    pub object: crate::grasping_msgs::msg::Object,
    pub grasps: Vec<crate::moveit_msgs::msg::Grasp>,
}

impl Default for GraspableObject {
    fn default() -> Self {
        GraspableObject {
            object: crate::grasping_msgs::msg::Object::default(),
            grasps: Vec::new(),
        }
    }
}
