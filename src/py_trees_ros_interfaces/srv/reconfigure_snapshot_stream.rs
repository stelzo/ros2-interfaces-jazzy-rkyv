use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReconfigureSnapshotStreamRequest {
    pub topic_name: ::std::string::String,
    pub parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters,
}

impl Default for ReconfigureSnapshotStreamRequest {
    fn default() -> Self {
        ReconfigureSnapshotStreamRequest {
            topic_name: ::std::string::String::new(),
            parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReconfigureSnapshotStreamResponse {
    pub result: bool,
}

impl Default for ReconfigureSnapshotStreamResponse {
    fn default() -> Self {
        ReconfigureSnapshotStreamResponse { result: false }
    }
}

pub struct ReconfigureSnapshotStream;
