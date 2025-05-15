use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Pose2D {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

impl Default for Pose2D {
    fn default() -> Self {
        Pose2D {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
}
