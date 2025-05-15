use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetCameraInfoRequest {
    pub camera_info: crate::sensor_msgs::msg::CameraInfo,
}

impl Default for SetCameraInfoRequest {
    fn default() -> Self {
        SetCameraInfoRequest {
            camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetCameraInfoResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetCameraInfoResponse {
    fn default() -> Self {
        SetCameraInfoResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetCameraInfo;
