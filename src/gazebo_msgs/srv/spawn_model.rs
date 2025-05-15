use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnModelRequest {
    pub model_name: ::std::string::String,
    pub model_xml: ::std::string::String,
    pub robot_namespace: ::std::string::String,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub reference_frame: ::std::string::String,
}

impl Default for SpawnModelRequest {
    fn default() -> Self {
        SpawnModelRequest {
            model_name: ::std::string::String::new(),
            model_xml: ::std::string::String::new(),
            robot_namespace: ::std::string::String::new(),
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnModelResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SpawnModelResponse {
    fn default() -> Self {
        SpawnModelResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SpawnModel;
