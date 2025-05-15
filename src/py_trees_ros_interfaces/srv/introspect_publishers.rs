use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IntrospectPublishersRequest {}

impl Default for IntrospectPublishersRequest {
    fn default() -> Self {
        IntrospectPublishersRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IntrospectPublishersResponse {
    pub publisher_details: Vec<crate::py_trees_ros_interfaces::msg::PublisherDetails>,
}

impl Default for IntrospectPublishersResponse {
    fn default() -> Self {
        IntrospectPublishersResponse {
            publisher_details: Vec::new(),
        }
    }
}

pub struct IntrospectPublishers;
