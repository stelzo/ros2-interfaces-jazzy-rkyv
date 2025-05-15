use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsPathValidRequest {
    pub path: crate::nav_msgs::msg::Path,
}

impl Default for IsPathValidRequest {
    fn default() -> Self {
        IsPathValidRequest {
            path: crate::nav_msgs::msg::Path::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsPathValidResponse {
    pub is_valid: bool,
    pub invalid_pose_indices: Vec<i32>,
}

impl Default for IsPathValidResponse {
    fn default() -> Self {
        IsPathValidResponse {
            is_valid: false,
            invalid_pose_indices: Vec::new(),
        }
    }
}

pub struct IsPathValid;
