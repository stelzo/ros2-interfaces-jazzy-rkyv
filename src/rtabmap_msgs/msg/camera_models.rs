use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CameraModels {
    pub models: Vec<crate::rtabmap_msgs::msg::CameraModel>,
}

impl Default for CameraModels {
    fn default() -> Self {
        CameraModels { models: Vec::new() }
    }
}
