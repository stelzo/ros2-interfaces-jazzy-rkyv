use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NormalizedImageCropRequest {
    pub top_left: crate::geometry_msgs::msg::Pose2D,
    pub bottom_right: crate::geometry_msgs::msg::Pose2D,
}

impl Default for NormalizedImageCropRequest {
    fn default() -> Self {
        NormalizedImageCropRequest {
            top_left: crate::geometry_msgs::msg::Pose2D::default(),
            bottom_right: crate::geometry_msgs::msg::Pose2D::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NormalizedImageCropResponse {
    pub status: i64,
}

impl Default for NormalizedImageCropResponse {
    fn default() -> Self {
        NormalizedImageCropResponse { status: 0 }
    }
}

pub struct NormalizedImageCrop;
