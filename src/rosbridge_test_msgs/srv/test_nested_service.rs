use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestNestedServiceRequest {
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for TestNestedServiceRequest {
    fn default() -> Self {
        TestNestedServiceRequest {
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestNestedServiceResponse {
    pub data: crate::std_msgs::msg::Float64,
}

impl Default for TestNestedServiceResponse {
    fn default() -> Self {
        TestNestedServiceResponse {
            data: crate::std_msgs::msg::Float64::default(),
        }
    }
}

pub struct TestNestedService;
