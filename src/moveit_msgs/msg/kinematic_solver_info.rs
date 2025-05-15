use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct KinematicSolverInfo {
    pub joint_names: Vec<::std::string::String>,
    pub limits: Vec<crate::moveit_msgs::msg::JointLimits>,
    pub link_names: Vec<::std::string::String>,
}

impl Default for KinematicSolverInfo {
    fn default() -> Self {
        KinematicSolverInfo {
            joint_names: Vec::new(),
            limits: Vec::new(),
            link_names: Vec::new(),
        }
    }
}
