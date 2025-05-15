use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Confidence {
    pub confidence: f32, // default: -1.0
}

impl Confidence {
    pub const CONFIDENCE_UNKNOWN: f32 = -1.0;
}

impl Default for Confidence {
    fn default() -> Self {
        Confidence { confidence: -1.0 }
    }
}
