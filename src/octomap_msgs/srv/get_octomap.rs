use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetOctomapRequest {}

impl Default for GetOctomapRequest {
    fn default() -> Self {
        GetOctomapRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetOctomapResponse {
    pub map: crate::octomap_msgs::msg::Octomap,
}

impl Default for GetOctomapResponse {
    fn default() -> Self {
        GetOctomapResponse {
            map: crate::octomap_msgs::msg::Octomap::default(),
        }
    }
}

pub struct GetOctomap;
