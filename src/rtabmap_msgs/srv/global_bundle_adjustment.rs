use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GlobalBundleAdjustmentRequest {
    pub r#type: i32,
    pub iterations: i32,
    pub pixel_variance: f32,
    pub voc_matches: bool,
}

impl Default for GlobalBundleAdjustmentRequest {
    fn default() -> Self {
        GlobalBundleAdjustmentRequest {
            r#type: 0,
            iterations: 0,
            pixel_variance: 0.0,
            voc_matches: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GlobalBundleAdjustmentResponse {}

impl Default for GlobalBundleAdjustmentResponse {
    fn default() -> Self {
        GlobalBundleAdjustmentResponse {}
    }
}

pub struct GlobalBundleAdjustment;
