use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IntrospectSubscribersRequest {}

impl Default for IntrospectSubscribersRequest {
    fn default() -> Self {
        IntrospectSubscribersRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IntrospectSubscribersResponse {
    pub subscriber_details: Vec<crate::py_trees_ros_interfaces::msg::SubscriberDetails>,
}

impl Default for IntrospectSubscribersResponse {
    fn default() -> Self {
        IntrospectSubscribersResponse {
            subscriber_details: Vec::new(),
        }
    }
}

pub struct IntrospectSubscribers;
