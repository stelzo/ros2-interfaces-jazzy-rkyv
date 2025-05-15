use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ShortVariedMultiNestedRequest {
    pub short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested,
}

impl Default for ShortVariedMultiNestedRequest {
    fn default() -> Self {
        ShortVariedMultiNestedRequest {
            short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ShortVariedMultiNestedResponse {
    pub bool_value: bool,
}

impl Default for ShortVariedMultiNestedResponse {
    fn default() -> Self {
        ShortVariedMultiNestedResponse { bool_value: false }
    }
}

pub struct ShortVariedMultiNested;
