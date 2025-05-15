use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DetectMoreLoopClosuresRequest {
    pub cluster_radius_max: f32,
    pub cluster_radius_min: f32,
    pub cluster_angle: f32,
    pub iterations: i32,
    pub intra_only: bool,
    pub inter_only: bool,
}

impl Default for DetectMoreLoopClosuresRequest {
    fn default() -> Self {
        DetectMoreLoopClosuresRequest {
            cluster_radius_max: 0.0,
            cluster_radius_min: 0.0,
            cluster_angle: 0.0,
            iterations: 0,
            intra_only: false,
            inter_only: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DetectMoreLoopClosuresResponse {
    pub detected: i32,
}

impl Default for DetectMoreLoopClosuresResponse {
    fn default() -> Self {
        DetectMoreLoopClosuresResponse { detected: 0 }
    }
}

pub struct DetectMoreLoopClosures;
