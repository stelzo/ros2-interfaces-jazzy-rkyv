use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NestedUint8ArrayTestServiceRequest {
    pub input: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage,
}

impl Default for NestedUint8ArrayTestServiceRequest {
    fn default() -> Self {
        NestedUint8ArrayTestServiceRequest {
            input: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NestedUint8ArrayTestServiceResponse {
    pub output: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage,
}

impl Default for NestedUint8ArrayTestServiceResponse {
    fn default() -> Self {
        NestedUint8ArrayTestServiceResponse {
            output: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage::default(
            ),
        }
    }
}

pub struct NestedUint8ArrayTestService;
