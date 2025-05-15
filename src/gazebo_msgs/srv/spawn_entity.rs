use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnEntityRequest {
    pub name: ::std::string::String,
    pub xml: ::std::string::String,
    pub robot_namespace: ::std::string::String,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub reference_frame: ::std::string::String,
}

impl Default for SpawnEntityRequest {
    fn default() -> Self {
        SpawnEntityRequest {
            name: ::std::string::String::new(),
            xml: ::std::string::String::new(),
            robot_namespace: ::std::string::String::new(),
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnEntityResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SpawnEntityResponse {
    fn default() -> Self {
        SpawnEntityResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SpawnEntity;
