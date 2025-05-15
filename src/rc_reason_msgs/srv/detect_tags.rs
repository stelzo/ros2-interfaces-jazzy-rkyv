use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DetectTagsRequest {
    pub tags: Vec<crate::rc_reason_msgs::msg::Tag>,
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectTagsRequest {
    fn default() -> Self {
        DetectTagsRequest {
            tags: Vec::new(),
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DetectTagsResponse {
    pub tags: Vec<crate::rc_reason_msgs::msg::DetectedTag>,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectTagsResponse {
    fn default() -> Self {
        DetectTagsResponse {
            tags: Vec::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct DetectTags;
