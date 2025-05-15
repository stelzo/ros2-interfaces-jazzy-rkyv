use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WorkspaceParameters {
    pub header: crate::std_msgs::msg::Header,
    pub min_corner: crate::geometry_msgs::msg::Vector3,
    pub max_corner: crate::geometry_msgs::msg::Vector3,
}

impl Default for WorkspaceParameters {
    fn default() -> Self {
        WorkspaceParameters {
            header: crate::std_msgs::msg::Header::default(),
            min_corner: crate::geometry_msgs::msg::Vector3::default(),
            max_corner: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}
