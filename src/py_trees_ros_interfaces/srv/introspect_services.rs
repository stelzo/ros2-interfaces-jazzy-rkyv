use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IntrospectServicesRequest {}

impl Default for IntrospectServicesRequest {
    fn default() -> Self {
        IntrospectServicesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IntrospectServicesResponse {
    pub service_details: Vec<crate::py_trees_ros_interfaces::msg::ServiceDetails>,
}

impl Default for IntrospectServicesResponse {
    fn default() -> Self {
        IntrospectServicesResponse {
            service_details: Vec::new(),
        }
    }
}

pub struct IntrospectServices;
