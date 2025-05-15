use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OctomapWithPose {
    pub header: crate::std_msgs::msg::Header,
    pub origin: crate::geometry_msgs::msg::Pose,
    pub octomap: crate::octomap_msgs::msg::Octomap,
}

impl Default for OctomapWithPose {
    fn default() -> Self {
        OctomapWithPose {
            header: crate::std_msgs::msg::Header::default(),
            origin: crate::geometry_msgs::msg::Pose::default(),
            octomap: crate::octomap_msgs::msg::Octomap::default(),
        }
    }
}
