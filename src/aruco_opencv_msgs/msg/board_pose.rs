use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BoardPose {
    pub board_name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for BoardPose {
    fn default() -> Self {
        BoardPose {
            board_name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}
