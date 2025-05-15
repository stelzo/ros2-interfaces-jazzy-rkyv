use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApplyBodyWrenchRequest {
    pub body_name: ::std::string::String,
    pub reference_frame: ::std::string::String,
    pub reference_point: crate::geometry_msgs::msg::Point,
    pub wrench: crate::geometry_msgs::msg::Wrench,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for ApplyBodyWrenchRequest {
    fn default() -> Self {
        ApplyBodyWrenchRequest {
            body_name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
            reference_point: crate::geometry_msgs::msg::Point::default(),
            wrench: crate::geometry_msgs::msg::Wrench::default(),
            start_time: crate::builtin_interfaces::msg::Time::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApplyBodyWrenchResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for ApplyBodyWrenchResponse {
    fn default() -> Self {
        ApplyBodyWrenchResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct ApplyBodyWrench;
