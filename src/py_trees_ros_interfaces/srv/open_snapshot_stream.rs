use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OpenSnapshotStreamRequest {
    pub topic_name: ::std::string::String,
    pub parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters,
}

impl Default for OpenSnapshotStreamRequest {
    fn default() -> Self {
        OpenSnapshotStreamRequest {
            topic_name: ::std::string::String::new(),
            parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OpenSnapshotStreamResponse {
    pub topic_name: ::std::string::String,
}

impl Default for OpenSnapshotStreamResponse {
    fn default() -> Self {
        OpenSnapshotStreamResponse {
            topic_name: ::std::string::String::new(),
        }
    }
}

pub struct OpenSnapshotStream;
