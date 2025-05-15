use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SilhouetteMatchDetectObjectRequest {
    pub object_to_detect: crate::rc_reason_msgs::msg::SilhouetteMatchObject,
    pub offset: f64,
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub load_carrier_id: ::std::string::String,
    pub collision_detection: crate::rc_reason_msgs::msg::CollisionDetection,
    pub object_plane_detection: bool,
}

impl Default for SilhouetteMatchDetectObjectRequest {
    fn default() -> Self {
        SilhouetteMatchDetectObjectRequest {
            object_to_detect: crate::rc_reason_msgs::msg::SilhouetteMatchObject::default(),
            offset: 0.0,
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            load_carrier_id: ::std::string::String::new(),
            collision_detection: crate::rc_reason_msgs::msg::CollisionDetection::default(),
            object_plane_detection: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SilhouetteMatchDetectObjectResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub object_id: ::std::string::String,
    pub matches: Vec<crate::rc_reason_msgs::msg::Match>,
    pub grasps: Vec<crate::rc_reason_msgs::msg::Grasp>,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SilhouetteMatchDetectObjectResponse {
    fn default() -> Self {
        SilhouetteMatchDetectObjectResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            object_id: ::std::string::String::new(),
            matches: Vec::new(),
            grasps: Vec::new(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct SilhouetteMatchDetectObject;
