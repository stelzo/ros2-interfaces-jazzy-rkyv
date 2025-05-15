use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Particle {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub weight: f64,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            pose: crate::geometry_msgs::msg::Pose::default(),
            weight: 0.0,
        }
    }
}
