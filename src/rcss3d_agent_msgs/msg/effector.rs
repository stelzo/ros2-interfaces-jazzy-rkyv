use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Effector {
    pub hinge_joint_vels: Vec<crate::rcss3d_agent_msgs::msg::HingeJointVel>,
    pub universal_joint_vels: Vec<crate::rcss3d_agent_msgs::msg::UniversalJointVel>,
    pub beams: Vec<crate::rcss3d_agent_msgs::msg::Beam>,
    pub says: Vec<crate::rcss3d_agent_msgs::msg::Say>,
}

impl Default for Effector {
    fn default() -> Self {
        Effector {
            hinge_joint_vels: Vec::new(),
            universal_joint_vels: Vec::new(),
            beams: Vec::new(),
            says: Vec::new(),
        }
    }
}
