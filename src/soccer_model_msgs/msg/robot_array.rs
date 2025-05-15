use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RobotArray {
    pub header: crate::std_msgs::msg::Header,
    pub robots: Vec<crate::soccer_model_msgs::msg::Robot>,
}

impl Default for RobotArray {
    fn default() -> Self {
        RobotArray {
            header: crate::std_msgs::msg::Header::default(),
            robots: Vec::new(),
        }
    }
}
