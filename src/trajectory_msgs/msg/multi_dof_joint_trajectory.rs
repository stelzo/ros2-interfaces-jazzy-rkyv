use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MultiDOFJointTrajectory {
    pub header: crate::std_msgs::msg::Header,
    pub joint_names: Vec<::std::string::String>,
    pub points: Vec<crate::trajectory_msgs::msg::MultiDOFJointTrajectoryPoint>,
}

impl Default for MultiDOFJointTrajectory {
    fn default() -> Self {
        MultiDOFJointTrajectory {
            header: crate::std_msgs::msg::Header::default(),
            joint_names: Vec::new(),
            points: Vec::new(),
        }
    }
}
