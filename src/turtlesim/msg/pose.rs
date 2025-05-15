use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub linear_velocity: f32,
    pub angular_velocity: f32,
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            linear_velocity: 0.0,
            angular_velocity: 0.0,
        }
    }
}
