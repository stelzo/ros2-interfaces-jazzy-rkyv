use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointTrajectory {
    pub header: crate::std_msgs::msg::Header,
    pub joint_names: Vec<::std::string::String>,
    pub points: Vec<crate::trajectory_msgs::msg::JointTrajectoryPoint>,
}

impl Default for JointTrajectory {
    fn default() -> Self {
        JointTrajectory {
            header: crate::std_msgs::msg::Header::default(),
            joint_names: Vec::new(),
            points: Vec::new(),
        }
    }
}
