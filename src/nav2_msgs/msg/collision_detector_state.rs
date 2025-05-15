use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CollisionDetectorState {
    pub polygons: Vec<::std::string::String>,
    pub detections: Vec<bool>,
}

impl Default for CollisionDetectorState {
    fn default() -> Self {
        CollisionDetectorState {
            polygons: Vec::new(),
            detections: Vec::new(),
        }
    }
}
