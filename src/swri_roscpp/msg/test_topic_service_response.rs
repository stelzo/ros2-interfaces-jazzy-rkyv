use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestTopicServiceResponse {
    pub srv_header: crate::marti_common_msgs::msg::ServiceHeader,
    pub response_value: i32,
}

impl Default for TestTopicServiceResponse {
    fn default() -> Self {
        TestTopicServiceResponse {
            srv_header: crate::marti_common_msgs::msg::ServiceHeader::default(),
            response_value: 0,
        }
    }
}
