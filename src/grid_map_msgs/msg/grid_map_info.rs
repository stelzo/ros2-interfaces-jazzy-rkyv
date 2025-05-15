use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GridMapInfo {
    pub resolution: f64,
    pub length_x: f64,
    pub length_y: f64,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for GridMapInfo {
    fn default() -> Self {
        GridMapInfo {
            resolution: 0.0,
            length_x: 0.0,
            length_y: 0.0,
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}
