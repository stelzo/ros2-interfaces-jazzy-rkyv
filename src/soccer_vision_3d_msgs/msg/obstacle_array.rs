use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObstacleArray {
    pub header: crate::std_msgs::msg::Header,
    pub obstacles: Vec<crate::soccer_vision_3d_msgs::msg::Obstacle>,
}

impl Default for ObstacleArray {
    fn default() -> Self {
        ObstacleArray {
            header: crate::std_msgs::msg::Header::default(),
            obstacles: Vec::new(),
        }
    }
}
